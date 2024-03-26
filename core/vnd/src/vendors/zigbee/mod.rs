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

use self::client::{Client, Payload, Topic};

use super::Vendors;

mod client;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Serialize, Deserialize)]
pub struct ZigbeeConfiguration {}

#[derive(Clone, Default)]
pub struct ZigbeeClass {}

#[allow(dead_code)]
pub struct Context<V>
where
	V: Vendor,
{
	mqtt: RwLock<mqtt::Client>,
	client: Client,
	arguments: SandboxArguments<V>,
}

#[async_trait]
impl Vendor for ZigbeeClass {
	type Configuration = ZigbeeConfiguration;
	type Context = Context<Self>;
	type PollData = (Topic, Payload);

	const NAME: &'static str = "zigbee";
	const VENDOR: Vendors = Vendors::Zigbee;

	const SUBSCRIBE_BUS: bool = false;
	const STOP_ON_ERROR: bool = true;

	async fn initialize(
		&self,
		arguments: SandboxArguments<Self>,
	) -> Result<Self::Context, SandboxError> {
		let name = arguments.component.id();

		let (mut tx, rx) =
			mqtt::client(name.to_raw().as_str()).map_err(|_| "Cannot create client")?;
		let _ = tx.subscribe("zigbee/#")?;

		Ok(Context {
			mqtt: RwLock::new((tx, rx)),
			client: Client {},
			arguments,
		})
	}

	async fn poll(&self, ctx: RefContext<Self>) -> Result<Option<Self::PollData>, SandboxError> {
		let mut client = ctx.mqtt.write().await;
		let notification = client.1.next().await?.ok_or("Link closed")?;

		let data = match notification {
			Notification::Forward(forward) => {
				let publish = forward.publish;
				let topic = String::from_utf8_lossy(&publish.topic);
				let payload = publish.payload;

				let topic: String = topic.chars().skip(7).collect();
				Some((topic.into(), payload.into()))
			}
			Notification::Disconnect(_, _) => {
				return Err("Link closed".into());
			}
			_ => None,
		};

		Ok(data)
	}

	async fn consume(
		&self,
		ctx: RefContext<Self>,
		data: Self::PollData,
	) -> Result<(), SandboxError> {
		let _ = ctx.client.handle(data.0, data.1).await;

		Ok(())
	}

	async fn services(&self) -> HashSet<ServiceType> {
		let mut set = HashSet::new();

		set.insert(ServiceType::new(ServiceData {}));
		set
	}

	/// Get the triggers for the vendor.
	async fn triggers(&self) -> HashSet<Trigger> {
		let mut set = HashSet::new();

		set.insert(Trigger::new("test".to_string(), Event::Time, trg::TriggerOrigin::System));
		set
	}
}
