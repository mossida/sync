use bus::{Consumer, Event};
use futures::{future, StreamExt};
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use tokio_util::sync::CancellationToken;
use tracing::trace;

use crate::Vendor;

pub struct Sandbox<V>
where
	V: Vendor,
{
	vendor: V,
}

impl<V> Sandbox<V>
where
	V: Vendor,
{
	pub fn new(vendor: V) -> Self {
		Self {
			vendor,
		}
	}
}

pub enum SandboxMessage<Message> {
	Event(Event),
	VendorMessage(Message),
	PollingInstant,
}

impl<Message> From<Event> for SandboxMessage<Message> {
	fn from(event: Event) -> Self {
		Self::Event(event)
	}
}

pub struct State<Component>
where
	Component: Vendor,
{
	#[allow(dead_code)]
	configuration: Component::Configuration,
	tokens: Vec<CancellationToken>,
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
		let bus = bus::get();
		let mut tokens = Vec::new();

		if Component::SUBSCRIBE_BUS {
			tokens.push(bus.subscribe().to_actor(myself.clone()));
		}

		if Component::POLLING_INTERVAL > 0 {
			// Filter time events and chunk them into polling intervals
			tokens.push(
				bus.subscribe()
					.filter(|e| future::ready(e.is_time()))
					.chunks(Component::POLLING_INTERVAL)
					.map(|_| SandboxMessage::PollingInstant)
					.to_actor(myself.clone()),
			);
		}

		Ok(State {
			tokens,
			configuration,
		})
	}

	async fn post_start(
		&self,
		_: ActorRef<Self::Msg>,
		_: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		let _ = self.vendor.services().await;
		let _ = self.vendor.triggers().await;

		Ok(())
	}

	async fn handle(
		&self,
		_: ActorRef<Self::Msg>,
		message: Self::Msg,
		_: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		match message {
			SandboxMessage::PollingInstant => {
				trace!("Polling event received");
			}
			SandboxMessage::Event(event) => {
				if Component::SUBSCRIBE_BUS {
					self.vendor.on_event(event).await;
				}
			}
			SandboxMessage::VendorMessage(msg) => self.vendor.on_message(msg).await,
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
