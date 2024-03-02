use bus::Event;

use mqtt::rumqttd::{local::LinkTx, protocol::Publish, Notification};
use ractor::{
	async_trait,
	factory::{Factory, FactoryMessage, Job},
	Actor, ActorProcessingErr, ActorRef,
};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::{component::Component, Vendor, VendorMessage};

use self::factory::Worker;

use super::Vendors;

mod factory;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Deserialize, Serialize)]
pub struct ZigbeeConfig {}

#[derive(Clone)]
pub struct ZigbeeClass {
	config: ZigbeeConfig,
}

pub struct State {
	tx: LinkTx,
	factory: ActorRef<FactoryMessage<u64, Publish>>,
	listener_handle: JoinHandle<()>,
}

#[async_trait]
impl Actor for ZigbeeClass {
	type Msg = ZigbeeMessage;
	type Arguments = ();
	type State = State;

	async fn pre_start(
		&self,
		myself: ActorRef<Self::Msg>,
		_: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		let name = myself.get_name().unwrap(); // Safe since every component is created with a name
		let (tx, mut rx) = mqtt::link(name.as_str()).await?;

		// Forward all messages to the actor bus
		let handle = tokio::spawn(async move {
			while let Ok(msg) = rx.next().await {
				match msg {
					Some(notification) => {
						let _ = myself.send_message(ZigbeeMessage::Notification(notification));
					}
					None => continue,
				};
			}
		});

		// Spawn factory that will process all updates from the zigbee topics
		let (factory, _) = Actor::spawn(
			Some(format!("{}-factory", name)),
			Factory {
				worker_count: 3,
				..Default::default()
			},
			Box::new(Worker {}),
		)
		.await?;

		Ok(State {
			tx,
			factory,
			listener_handle: handle,
		})
	}

	async fn post_start(
		&self,
		_myself: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		// subscribe to main topic
		state.tx.subscribe("zigbee/#")?;

		Ok(())
	}

	async fn handle(
		&self,
		_myself: ActorRef<Self::Msg>,
		message: Self::Msg,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		match message {
			ZigbeeMessage::VendorMessage(_) => todo!(),
			ZigbeeMessage::Notification(notification) => match notification {
				Notification::Forward(forward) => {
					// Forward the message to the factory
					state.factory.send_message(FactoryMessage::Dispatch(Job {
						key: 0,
						msg: forward.publish,
						options: Default::default(),
					}))?;
				}
				_ => {
					dbg!("other");
				}
			},
		};

		Ok(())
	}

	async fn post_stop(
		&self,
		_: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		state.listener_handle.abort();

		Ok(())
	}
}

impl Vendor for ZigbeeClass {
	type Configuration = ZigbeeConfig;
	type Message = ZigbeeMessage;

	const NAME: &'static str = "zigbee";
	const VENDOR: Vendors = Vendors::Zigbee;

	fn new(config: Self::Configuration) -> Self {
		Self {
			config,
		}
	}

	fn configuration(&self) -> Self::Configuration {
		self.config.clone()
	}
}

pub enum ZigbeeMessage {
	VendorMessage(VendorMessage),
	Notification(Notification),
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
