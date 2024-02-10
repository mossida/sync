use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use resources::database;

use crate::entity;
use crate::entity::Entity;

pub const RESOURCE: &str = "device";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
	pub id: Thing,
	pub name: String,
	pub serial: String,
	pub model: String,
	pub manufacturer: String,
	pub sw_version: Option<String>,
	pub hw_version: Option<String>,
}

impl Device {
	pub async fn find(id: &String) -> utils::types::Result<Option<Device>> {
		let client = database::get();
		let device: Option<Device> = client
			.query(r#"SELECT * FROM $resource WHERE id = $id"#)
			.bind(("resource", RESOURCE))
			.bind(("id", id))
			.await?
			.take(0)?;

		Ok(device)
	}

	pub async fn updates(&self, entity: &Entity) -> utils::types::Result<()> {
		let client = database::get();

		if !entity.exists().await? {
			let _: Vec<Entity> = client.create(entity::RESOURCE).content(entity).await?;
		}

		client
			.query(r#"RELATE $device->updates->$entity"#)
			.bind(("device", &self.id))
			.bind(("entity", &entity.id))
			.await?;

		Ok(())
	}

	pub async fn get_entities(&self) -> utils::types::Result<Vec<Entity>> {
		let client = database::get();
		let entities: Vec<Entity> = client
			.query(r#"SELECT ->updates->entity FROM $device"#)
			.bind(("device", &self.id))
			.await?
			.take(0)?;

		Ok(entities)
	}
}
