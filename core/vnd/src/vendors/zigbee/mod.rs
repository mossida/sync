use bus::Event;
use serde::{Deserialize, Serialize};

use crate::{component::Component, Vendor, VendorMessage};

use super::Vendors;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct ZigbeeConfiguration {}

#[derive(Clone, Default)]
pub struct ZigbeeClass {}

impl Vendor for ZigbeeClass {
	type Configuration = ZigbeeConfiguration;
	type Message = ZigbeeMessage;

	const NAME: &'static str = "zigbee";
	const VENDOR: Vendors = Vendors::Zigbee;
	const SUBSCRIBE_BUS: bool = false;
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
