use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use resources::database;

use crate::attributes::Attributes;

pub const RESOURCE: &str = "entity";

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub id: Thing,
    pub enabled: bool,
    pub available: bool,
    pub class: String,
    pub attributes: Attributes,
    pub status: EntityState,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EntityState {
    pub state: String,
    pub attributes: Attributes,
    pub updated_at: surrealdb::sql::Datetime,
}

impl Entity {
    pub async fn exists(&self) -> utils::types::Result<bool> {
        let client = database::get();
        let entity: Option<Entity> = client.select(&self.id).await?;
        Ok(entity.is_some())
    }

    pub async fn list() -> utils::types::Result<Vec<Entity>> {
        let client = database::get();
        let entities: Vec<Entity> = client.select(RESOURCE).await?;
        Ok(entities)
    }

    pub async fn merge(&mut self, state: EntityState) -> utils::types::Result<()> {
        let client = database::get();

        let result: Option<Entity> = client
            .query("UPDATE $entity MERGE { status: $content }")
            .bind(("entity", &self.id))
            .bind(("content", &state))
            .await?
            .take(0)?;

        if let Some(entity) = result {
            self.status = entity.status;
        }

        Ok(())
    }
}
