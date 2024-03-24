use std::{borrow::BorrowMut, collections::HashSet, time::Duration};

use bus::Event;
use dbm::resource::Resource;

use mqtt::Notification;
use ractor::async_trait;
use serde::{Deserialize, Serialize};
use svc::r#type::{ServiceData, ServiceType};
use trg::Trigger;

use crate::{
	component::Component,
	sandbox::{actor::SandboxArguments, SandboxError},
	Vendor,
};

use super::Vendors;

mod factory;
mod payload;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct ZigbeeConfiguration {}

#[derive(Clone, Default)]
pub struct ZigbeeClass {}

pub struct Context {
	client: mqtt::Client,
}

#[async_trait]
impl Vendor for ZigbeeClass {
	type Configuration = ZigbeeConfiguration;
	type Context = Context;
	type PollData = Notification;

	const NAME: &'static str = "zigbee";
	const VENDOR: Vendors = Vendors::Zigbee;

	const SUBSCRIBE_BUS: bool = false;
	const POLLING_INTERVAL: Duration = Duration::from_millis(1);

	async fn initialize(
		&self,
		args: &SandboxArguments<Self>,
	) -> Result<Self::Context, SandboxError> {
		let name = args.component.id();

		let (mut tx, rx) = mqtt::client(name.to_raw().as_str()).unwrap();
		let _ = tx.subscribe("zigbee/#");

		Ok(Context {
			client: (tx, rx),
		})
	}

	async fn poll(&self, ctx: &mut Self::Context) -> Result<Self::PollData, SandboxError> {
		let rx = ctx.client.1.borrow_mut();
		let notification = rx.next().await?.ok_or("Link closed")?;

		Ok(notification)
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
