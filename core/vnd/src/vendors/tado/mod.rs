use bus::Event;
use serde::{Deserialize, Serialize};

use crate::{component::Component, Vendor, VendorMessage};

use super::Vendors;

pub type Tado = Component<TadoVendor>;

#[derive(Clone, Hash, Deserialize, Serialize)]
pub struct TadoConfig {}

#[derive(Clone, Default)]
pub struct TadoVendor {}

impl Vendor for TadoVendor {
	type Configuration = TadoConfig;
	type Message = TadoMessage;

	const NAME: &'static str = "tado";
	const VENDOR: Vendors = Vendors::Tado;
	const SUBSCRIBE_BUS: bool = false;
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
