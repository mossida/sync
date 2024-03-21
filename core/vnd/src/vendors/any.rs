use crate::{component::Component, Vendor, VendorMessage};

use super::Vendors;

pub type Any = Component<AnyVendor>;

#[derive(Debug, Clone, Default)]
pub struct AnyVendor {}

pub enum AnyMessage {}

impl Vendor for AnyVendor {
	type Configuration = ();
	type Message = AnyMessage;

	const NAME: &'static str = "any";
	const VENDOR: Vendors = Vendors::Any;
	const SUBSCRIBE_BUS: bool = false;
}

impl From<VendorMessage> for AnyMessage {
	fn from(_: VendorMessage) -> Self {
		unreachable!()
	}
}

impl From<bus::Event> for AnyMessage {
	fn from(_: bus::Event) -> Self {
		unreachable!()
	}
}
