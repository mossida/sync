use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use resources::database;

use crate::device;
use crate::device::Device;

pub const RESOURCE: &str = "component";

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Component {
	pub id: Thing,
	pub vendor: serde_json::Value,
	pub configuration: serde_json::Value,
	pub priority: u8,
}

impl Component {
	pub async fn controls(&self, device: &Device) -> utils::types::Result<()> {
		let client = database::get();

		client
			.query("INSERT INTO $resource $content")
			.bind(("resource", device::RESOURCE))
			.bind(("content", device))
			.await?;

		client
			.query(r#"RELATE $component->controls->$device"#)
			.bind(("component", &self.id))
			.bind(("device", &device.id))
			.await?;

		Ok(())
	}

	pub async fn get_devices(&self) -> utils::types::Result<Vec<Device>> {
		let client = database::get();
		let devices: Vec<Device> = client
			.query(
				r#"(SELECT ->controls->device as devices FROM ONLY $component FETCH devices).devices"#,
			)
			.bind(("component", &self.id))
			.await?
			.take(0)?;

		Ok(devices)
	}
}

impl Hash for Component {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}
