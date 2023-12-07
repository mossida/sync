use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;

use crate::devices::models::DeviceId;
use crate::integrations::classes::Class;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityId(RecordId);

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityAttributes {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub state: String,
    pub enabled: bool,
    pub available: bool,
    pub class: Class,
    pub attributes: EntityAttributes,
}

impl NewId for EntityId {
    const TABLE: &'static str = "entity";

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        EntityId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }

    fn get_inner_string(&self) -> String {
        self.0.to_string()
    }
}

pub trait EntityFactory {
    fn build_entity(device_id: DeviceId) -> Entity;
}
