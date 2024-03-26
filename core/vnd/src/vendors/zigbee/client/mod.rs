use std::ops::Deref;

use tokio_util::bytes::Bytes;

use crate::sandbox::SandboxError;

use self::topics::{bridge_devices::BridgeDevices, bridge_groups::BridgeGroups};

pub mod payload;
pub mod topics;

#[derive(Debug)]
pub enum Topic {
	BridgeDevices,
	BridgeGroups,
	Unknown,
}

impl From<String> for Topic {
	fn from(topic: String) -> Self {
		match topic.as_str() {
			"bridge/devices" => Topic::BridgeDevices,
			"bridge/groups" => Topic::BridgeGroups,
			_ => Topic::Unknown,
		}
	}
}

#[derive(Debug)]
pub struct Payload(Bytes);

impl From<Bytes> for Payload {
	fn from(value: Bytes) -> Self {
		Self(value)
	}
}

impl Deref for Payload {
	type Target = Bytes;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Clone)]
pub struct Client {}

impl Client {
	pub async fn handle(&self, topic: Topic, payload: Payload) -> Result<(), SandboxError> {
		match topic {
			Topic::BridgeDevices => BridgeDevices::new(self, payload)?.await,
			Topic::BridgeGroups => BridgeGroups::new(self, payload)?.await,
			Topic::Unknown => todo!(),
		}
	}
}
