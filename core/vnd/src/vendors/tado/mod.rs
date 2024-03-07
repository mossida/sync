use bus::{Consumer, Event};
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{component::Component, Vendor, VendorMessage};

use super::Vendors;

pub type Tado = Component<TadoVendor>;

#[derive(Clone, Hash, Deserialize, Serialize)]
pub struct TadoConfig {}

#[derive(Clone)]
pub struct TadoVendor {
	config: TadoConfig,
}

#[async_trait]
impl Actor for TadoVendor {
	type Msg = TadoMessage;
	type Arguments = ();
	type State = ();

	async fn pre_start(
		&self,
		myself: ActorRef<Self::Msg>,
		_: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		let bus = bus::get();
		let _ = bus.subscribe().to_actor(myself);

		info!("Tado actor started");

		Ok(())
	}
}

impl Vendor for TadoVendor {
	type Configuration = TadoConfig;
	type Message = TadoMessage;

	const NAME: &'static str = "tado";
	const VENDOR: Vendors = Vendors::Tado;

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
