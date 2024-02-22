use bus::Event;
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};

use crate::{Class, Vendor, VendorMessage};

pub type Tado = Vendor<TadoClass>;

#[derive(Clone)]
pub struct TadoClass {
	config: (),
}

#[async_trait]
impl Actor for TadoClass {
	type Msg = TadoMessage;
	type Arguments = ();
	type State = ();

	async fn pre_start(
		&self,
		myself: ActorRef<Self::Msg>,
		_: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		let bus = bus::get();
		bus.subscribe_actor(myself);

		Ok(())
	}
}

impl Class for TadoClass {
	type Configuration = ();
	type Message = TadoMessage;

	const NAME: &'static str = "tado";

	fn new(config: Self::Configuration) -> Self {
		Self {
			config,
		}
	}

	fn configuration(&self) -> Self::Configuration {
		self.config.clone()
	}
}

pub enum TadoMessage {
	BusEvent(Event),
}

impl From<VendorMessage> for TadoMessage {
	fn from(_: VendorMessage) -> Self {
		todo!()
	}
}

impl From<Event> for TadoMessage {
	fn from(event: Event) -> Self {
		TadoMessage::BusEvent(event)
	}
}
