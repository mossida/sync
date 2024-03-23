use bus::Consumer;
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use svc::r#type::ServiceType;

use tokio_util::sync::CancellationToken;
use tracing::warn;

use crate::Vendor;

use super::{context::Context, Request, Response, Sandbox, SandboxMessage};

pub struct State<Component>
where
	Component: Vendor,
{
	#[allow(dead_code)]
	configuration: Component::Configuration,
	tokens: Vec<CancellationToken>,
	services: Vec<ServiceType>,
	worker: ActorRef<()>,
	panics: u8,
}

#[async_trait]
impl<Component> Actor for Sandbox<Component>
where
	Component: Vendor,
{
	type Msg = SandboxMessage<Component::Message>;
	type Arguments = Component::Configuration;
	type State = State<Component>;

	async fn pre_start(
		&self,
		myself: ActorRef<Self::Msg>,
		configuration: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		// Setup vendor before listening to events
		self.vendor.initialize(configuration.clone()).await;

		let mut tokens = Vec::new();

		if Component::SUBSCRIBE_BUS {
			let bus = bus::get();
			tokens.push(bus.subscribe().to_actor(myself.clone()));
		}

		let worker = self.spawn_worker(myself.get_cell()).await?;
		let services = self.vendor.services().await;

		Ok(State {
			tokens,
			configuration,
			services,
			worker,
			panics: 0,
		})
	}

	async fn post_start(
		&self,
		_: ActorRef<Self::Msg>,
		_: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		// TODO: Register services

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
			SandboxMessage::VendorMessage(msg) => __self.vendor.on_message(&Context {}, msg).await,
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
