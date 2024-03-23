use std::time::Duration;

use bus::Event;
use ractor::async_trait;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{component::Component, Vendor, VendorMessage};

use super::Vendors;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct ZigbeeConfiguration {}

#[derive(Clone, Default)]
pub struct ZigbeeClass {}

#[async_trait]
impl Vendor for ZigbeeClass {
	type Configuration = ZigbeeConfiguration;
	type Message = ZigbeeMessage;

	const NAME: &'static str = "zigbee";
	const VENDOR: Vendors = Vendors::Zigbee;

	const SUBSCRIBE_BUS: bool = false;
	const POLLING_INTERVAL: u64 = 1;

	async fn run(&self) {
		debug!("Run called");
		tokio::time::sleep(Duration::from_secs(4)).await;
		debug!("After 4 seconds");
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
