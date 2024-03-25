use std::collections::HashSet;

use bus::Event;
use dbm::resource::Resource;

use mqtt::Notification;
use ractor::async_trait;
use serde::{Deserialize, Serialize};
use svc::r#type::{ServiceData, ServiceType};
use tokio::sync::RwLock;
use trg::Trigger;

use crate::{
	component::Component,
	sandbox::{actor::SandboxArguments, SandboxError},
	RefContext, Vendor,
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
	client: RwLock<mqtt::Client>,
}

#[async_trait]
impl Vendor for ZigbeeClass {
	type Configuration = ZigbeeConfiguration;
	type Context = Context;
	type PollData = Notification;

	const NAME: &'static str = "zigbee";
	const VENDOR: Vendors = Vendors::Zigbee;

	const SUBSCRIBE_BUS: bool = false;

	async fn initialize(
		&self,
		args: &SandboxArguments<Self>,
	) -> Result<Self::Context, SandboxError> {
		let name = args.component.id();

		let (mut tx, rx) =
			mqtt::client(name.to_raw().as_str()).map_err(|_| "Cannot create client")?;
		let _ = tx.subscribe("zigbee/#")?;

		Ok(Context {
			client: RwLock::new((tx, rx)),
		})
	}

	async fn poll(&self, ctx: RefContext<Self>) -> Result<Self::PollData, SandboxError> {
		let mut client = ctx.client.write().await;
		let notification = client.1.next().await?.ok_or("Link closed")?;

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
