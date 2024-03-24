use std::collections::HashSet;

use bus::Consumer;
use futures::future::join_all;
use ractor::{async_trait, pg, Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use svc::r#type::ServiceType;

use dbm::resource::Resource;
use tokio_util::sync::CancellationToken;
use tracing::warn;
use trg::Trigger;

use crate::{component::Component as ComponentInstance, Vendor, SANDBOX_GROUP, SCOPE};

use super::{Request, Response, Sandbox, SandboxMessage};

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
	component: ComponentInstance<Component>,
	configuration: Component::Configuration,
	tokens: Vec<CancellationToken>,
	services: HashSet<ServiceType>,
	triggers: HashSet<Trigger>,
	worker: ActorRef<()>,
	panics: u8,
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
		args: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		// Join the sandbox group
		pg::join_scoped(SCOPE.to_string(), SANDBOX_GROUP.to_string(), vec![myself.get_cell()]);

		let SandboxArguments {
			component,
			configuration,
		} = args;

		// Setup vendor before listening to events
		self.vendor.initialize(configuration.clone()).await;

		let mut tokens = Vec::new();

		if Component::SUBSCRIBE_BUS {
			let bus = bus::get();
			tokens.push(bus.subscribe().to_actor(myself.clone()));
		}

		let worker = self.spawn_worker(myself.get_cell()).await?;

		let services = self.vendor.services().await;
		let triggers = self.vendor.triggers(&component).await;

		Ok(State {
			tokens,
			configuration,
			component,
			services,
			triggers,
			worker,
			panics: 0,
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
		myself: ActorRef<Self::Msg>,
		message: SupervisionEvent,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		if let SupervisionEvent::ActorPanicked(_, _) = message {
			if state.panics >= Component::RETRIES {
				warn!("Worker actor panicked {} times, giving up...", state.panics);
				return Ok(());
			}

			warn!("Worker actor panicked! This should not happen, restarting...");
			// Restart worker
			state.panics += 1;
			state.worker = self.spawn_worker(myself.get_cell()).await?;
		}

		Ok(())
	}

	async fn post_stop(
		&self,
		_: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		state.worker.kill();
		state.tokens.iter().for_each(|t| t.cancel());

		self.vendor.stop().await;

		Ok(())
	}
}
