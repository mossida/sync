use ractor::{async_trait, Actor};

use crate::{component::Component, Vendor, VendorMessage};

use super::Vendors;

pub type Any = Component<AnyVendor>;

#[derive(Debug, Clone)]
pub struct AnyVendor {}

pub enum AnyMessage {}

#[async_trait]
impl Actor for AnyVendor {
	type Msg = AnyMessage;
	type Arguments = ();
	type State = ();

	async fn pre_start(
		&self,
		_: ractor::ActorRef<Self::Msg>,
		_: Self::Arguments,
	) -> Result<Self::State, ractor::ActorProcessingErr> {
		unreachable!()
	}
}

impl Vendor for AnyVendor {
	type Configuration = ();
	type Message = AnyMessage;

	const NAME: &'static str = "any";
	const VENDOR: Vendors = Vendors::Any;

	fn new(_: Self::Configuration) -> Self {
		unreachable!()
	}

	fn configuration(&self) -> Self::Configuration {
		unreachable!()
	}
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
