use bus::Event;

use dbm::Id;
use mqtt::rumqttd::{local::LinkTx, Notification};
use ractor::{
	async_trait,
	factory::{Factory, FactoryMessage, Job},
	Actor, ActorProcessingErr, ActorRef,
};
use serde::{Deserialize, Serialize};
use tokio::task::AbortHandle;
use tracing::{span, trace, Level};

use crate::{component::Component, Vendor, VendorMessage};

use self::factory::Worker;

use super::Vendors;

mod factory;
mod payload;

pub type Zigbee = Component<ZigbeeClass>;

#[derive(Clone, Hash, Deserialize, Serialize)]
pub struct ZigbeeConfig {}

#[derive(Clone)]
pub struct ZigbeeClass {
	config: ZigbeeConfig,
}

pub struct State {
	tx: LinkTx,
	handles: Vec<AbortHandle>,
}

#[async_trait]
impl Actor for ZigbeeClass {
	type Msg = ZigbeeMessage;
	type Arguments = ();
	type State = State;

	/// Pre start function that will create
	/// a mqtt client, spawn the factory of workers
	/// and forward all the notifications from mqtt to the factory.
	///
	/// These 3 components are critical for the Zigbee component to work.
	/// So if one of the fails the actor will not start.
	async fn pre_start(
		&self,
		myself: ActorRef<Self::Msg>,
		_: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		// Use component name or generate a random one
		let name = myself.get_name().unwrap_or(Id::rand().to_raw());
		let (tx, mut rx) = mqtt::link(name.as_str()).await?;

		// Spawn factory that will process all updates from the zigbee topics
		let (factory, factory_handle) = Actor::spawn(
			Some(format!("{}-factory", name)),
			Factory {
				worker_count: 3,
				..Default::default()
			},
			Box::new(Worker {}),
		)
		.await?;

		// Forward all messages to the actor bus
		let inner_factory = factory.clone();
		let span = span!(Level::TRACE, "zigbee");
		let handle = tokio::spawn(async move {
			let _ = span.enter();

			while let Ok(msg) = rx.next().await {
				match msg {
					Some(notification) => {
						if let Notification::Forward(forward) = notification {
							// Forward the message to the factory
							let publish = forward.publish;
							let topic_conversion = String::from_utf8(publish.topic.to_vec());
							let payload_conversion = String::from_utf8(publish.payload.to_vec());

							if topic_conversion.is_err() || payload_conversion.is_err() {
								continue;
							}

							let topic = topic_conversion.unwrap();
							let payload = payload_conversion.unwrap();

							trace!("Forwarding message to factory from topic: {}", topic);
							let _ = inner_factory.send_message(FactoryMessage::Dispatch(Job {
								key: topic,
								msg: payload,
								options: Default::default(),
							}));
						}
					}
					None => continue,
				};
			}
		});

		Ok(State {
			tx,
			handles: vec![handle.abort_handle(), factory_handle.abort_handle()],
		})
	}

	/// Post start function of the component
	/// Subscribes to the MQTT topic and start logs
	async fn post_start(
		&self,
		_myself: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		// subscribe to main topic
		state.tx.subscribe("zigbee/#")?;

		Ok(())
	}

	/// Handle function that will process all the external messages
	/// like all the vendors messages, automations and bus
	async fn handle(
		&self,
		_myself: ActorRef<Self::Msg>,
		_: Self::Msg,
		_: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		// Process all external messages

		Ok(())
	}

	/// Post stop function that will stop all the handles
	async fn post_stop(
		&self,
		_: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		for handle in state.handles.drain(..) {
			handle.abort();
		}

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
