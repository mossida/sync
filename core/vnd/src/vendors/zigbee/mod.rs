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

mod payload;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct ZigbeeConfiguration {}

#[derive(Clone, Default)]
pub struct ZigbeeClass {}

pub struct Context {
	client: RwLock<mqtt::Client>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
	topic: String,
	payload: String,
}

#[async_trait]
impl Vendor for ZigbeeClass {
	type Configuration = ZigbeeConfiguration;
	type Context = Context;
	type PollData = Data;

	const NAME: &'static str = "zigbee";
	const VENDOR: Vendors = Vendors::Zigbee;

	const SUBSCRIBE_BUS: bool = false;
	const STOP_ON_ERROR: bool = true;

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

	async fn poll(&self, ctx: RefContext<Self>) -> Result<Option<Self::PollData>, SandboxError> {
		let mut client = ctx.client.write().await;
		let notification = client.1.next().await?.ok_or("Link closed")?;

		let data = match notification {
			Notification::Forward(forward) => {
				let publish = forward.publish;
				let topic = String::from_utf8(publish.topic.to_vec());
				let payload = String::from_utf8(publish.payload.to_vec());

				match (topic, payload) {
					(Ok(topic), Ok(payload)) => Some(Data {
						topic,
						payload,
					}),
					_ => None,
				}
			}
			Notification::Disconnect(_, _) => {
				return Err("Link closed".into());
			}
			_ => None,
		};

		Ok(data)
	}

	async fn consume(&self, _: RefContext<Self>, data: Self::PollData) -> Result<(), SandboxError> {
		dbg!(data);

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
