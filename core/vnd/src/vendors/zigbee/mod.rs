use bus::Event;
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use serde::{Deserialize, Serialize};

use crate::{component::Component, Vendor, VendorMessage};

use super::Vendors;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct ZigbeeConfiguration {}

#[derive(Clone)]
pub struct ZigbeeClass {
	config: ZigbeeConfiguration,
}

#[async_trait]
impl Actor for ZigbeeClass {
	type Msg = ZigbeeMessage;
	type Arguments = ();
	type State = ();

	async fn pre_start(
		&self,
		_: ActorRef<Self::Msg>,
		_: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		Ok(())
	}
}

impl Vendor for ZigbeeClass {
	type Configuration = ZigbeeConfiguration;
	type Message = ZigbeeMessage;

	const NAME: &'static str = "zigbee";
	const VENDOR: Vendors = Vendors::Zigbee;

	fn new(config: Self::Configuration) -> Self {
		Self {
			config,
		}
	}

	fn configuration(&self) -> Self::Configuration {
		self.config.clone()
	}
}

pub enum ZigbeeMessage {}

impl From<VendorMessage> for ZigbeeMessage {
	fn from(_: VendorMessage) -> Self {
		todo!()
	}
}

impl From<Event> for ZigbeeMessage {
	fn from(_: Event) -> Self {
		todo!()
	}
}
