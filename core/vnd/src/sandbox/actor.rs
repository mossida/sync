use std::{collections::HashSet, sync::Arc};

use bus::Consumer;
use futures::future::join_all;
use ractor::{
	async_trait,
	factory::{Factory, FactoryMessage, Job, RoutingMode},
	pg, Actor, ActorProcessingErr, ActorRef, SupervisionEvent,
};
use svc::r#type::ServiceType;

use dbm::resource::Resource;
use tokio_util::sync::CancellationToken;
use tracing::error;
use trg::Trigger;

use crate::{component::Component as ComponentInstance, Vendor, SANDBOX_GROUP, SCOPE};

use super::{
	worker::{Builder, Task},
	Request, Response, Sandbox, SandboxMessage,
};

#[derive(Clone)]
pub struct SandboxArguments<Component>
where
	Component: Vendor,
{
	pub component: ComponentInstance<Component>,
	pub configuration: Component::Configuration,
}

#[allow(dead_code)]
pub struct State<Component>
where
	Component: Vendor,
{
	arguments: SandboxArguments<Component>,
	tokens: Vec<CancellationToken>,
	services: HashSet<ServiceType>,
	triggers: HashSet<Trigger>,
	factory: ActorRef<FactoryMessage<Task, Option<Component::PollData>>>,
}

#[async_trait]
impl<Component> Actor for Sandbox<Component>
where
	Component: Vendor,
{
	type Msg = SandboxMessage;
	type Arguments = SandboxArguments<Component>;
	type State = State<Component>;

	async fn pre_start(
		&self,
		myself: ActorRef<Self::Msg>,
		arguments: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		// Join the sandbox group
		pg::join_scoped(SCOPE.to_string(), SANDBOX_GROUP.to_string(), vec![myself.get_cell()]);

		// Setup vendor before listening to events
		let raw_context = self.vendor.initialize(&arguments).await?;
		let context: Arc<_> = Arc::new(raw_context);
		let mut tokens = Vec::new();

		if Component::SUBSCRIBE_BUS {
			let bus = bus::get();
			tokens.push(bus.subscribe().to_actor(myself.clone()));
		}

		let (factory, _) = Actor::spawn_linked(
			None,
			Factory {
				worker_count: 2,
				worker_parallel_capacity: 1,
				collect_worker_stats: false,
				routing_mode: RoutingMode::StickyQueuer,
				discard_threshold: Some(10),
				dead_mans_switch: None,
				..Default::default()
			},
			Box::new(Builder {
				vendor: self.vendor.clone(),
				context,
			}),
			myself.get_cell(),
		)
		.await?;

		factory.send_interval(Component::POLLING_INTERVAL, || {
			FactoryMessage::Dispatch(Job {
				key: Task::Poll,
				msg: None,
				options: Default::default(),
			})
		});

		let services = self.vendor.services().await;
		let triggers = self.vendor.triggers(&arguments.component).await;

		Ok(State {
			factory,
			arguments,
			tokens,
			services,
			triggers,
		})
	}

	async fn post_start(
		&self,
		_: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		// TODO: Register services
		let triggers: Vec<_> = state.triggers.iter().map(Resource::create).collect();
		let services: Vec<_> = state.services.iter().map(Resource::create).collect();

		join_all(triggers).await;
		join_all(services).await;

		Ok(())
	}

	async fn handle(
		&self,
		_: ActorRef<Self::Msg>,
		message: Self::Msg,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		match message {
			SandboxMessage::Event(event) => self.vendor.on_event(event).await,
			SandboxMessage::Request(rq, rpc) => match rq {
				Request::Call(service) => {
					let is_registered = state.services.iter().any(|s| service.is(s));

					match is_registered {
						true => {
							let call_result = self.vendor.on_service_call(service).await;
							let _ = rpc.send(call_result.into());
						}
						false => {
							let _ = rpc.send(Response::NotHandled);
						}
					};
				}
			},
		};

		Ok(())
	}

	async fn handle_supervisor_evt(
		&self,
		_: ActorRef<Self::Msg>,
		message: SupervisionEvent,
		_: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		match message {
			SupervisionEvent::ActorTerminated(_, _, _) | SupervisionEvent::ActorPanicked(_, _) => {
				// TODO: Handle factory termination
				error!("Polling system has terminated");
			}
			_ => {}
		}
		Ok(())
	}

	async fn post_stop(
		&self,
		_: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		state.factory.kill();
		state.tokens.iter().for_each(|t| t.cancel());

		self.vendor.stop().await;

		Ok(())
	}
}
