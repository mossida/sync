use std::{collections::HashSet, time::Duration};

use bus::Event;
use ractor::async_trait;
use serde::{Deserialize, Serialize};
use svc::r#type::{ServiceData, ServiceType};
use tracing::debug;
use trg::Trigger;

use crate::{component::Component, sandbox::SandboxError, Vendor, VendorMessage};

use super::Vendors;

mod factory;
mod payload;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Deserialize, Serialize)]
pub struct ZigbeeConfig {}

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
	const POLLING_INTERVAL: Duration = Duration::from_secs(0);

	async fn poll(&self) -> Result<(), SandboxError> {
		debug!("Run called");
		tokio::time::sleep(Duration::from_secs(4)).await;
		debug!("After 4 seconds");

		Ok(())
	}

	async fn services(&self) -> HashSet<ServiceType> {
		let mut set = HashSet::new();

		set.insert(ServiceType::new(ServiceData {}));
		set
	}

	/// Get the triggers for the vendor.
	async fn triggers(&self, _: &Component<Self>) -> HashSet<Trigger> {
		let mut set = HashSet::new();

		set.insert(Trigger::new("test".to_string(), Event::Time, trg::TriggerOrigin::System));
		set
	}
}

pub enum ZigbeeMessage {
	VendorMessage(VendorMessage),
}

impl From<VendorMessage> for ZigbeeMessage {
	fn from(m: VendorMessage) -> Self {
		ZigbeeMessage::VendorMessage(m)
	}
}

impl From<Event> for ZigbeeMessage {
	fn from(_: Event) -> Self {
		todo!()
	}
}
