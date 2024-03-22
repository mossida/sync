use std::future;

use bus::Consumer;
use futures::StreamExt;
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use svc::r#type::ServiceType;
use tokio_util::sync::CancellationToken;
use tracing::trace;

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
		self.vendor.setup(configuration.clone()).await;

		let bus = bus::get();
		let mut tokens = Vec::new();

		if Component::SUBSCRIBE_BUS {
			tokens.push(bus.subscribe().to_actor(myself.clone()));
		}

		if Component::POLLING_INTERVAL > 0 {
			// Filter time events and chunk them into polling intervals
			let token = bus
				.subscribe()
				.filter(|e| future::ready(e.is_time()))
				.chunks(Component::POLLING_INTERVAL)
				.map(|_| SandboxMessage::PollingInstant)
				.to_actor(myself.clone());

			tokens.push(token);
		}

		let services = self.vendor.services().await;

		Ok(State {
			tokens,
			configuration,
			services,
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
			SandboxMessage::PollingInstant => {
				trace!("Polling event received");
			}
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

	async fn post_stop(
		&self,
		_: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		state.tokens.iter().for_each(|t| t.cancel());

		Ok(())
	}
}
