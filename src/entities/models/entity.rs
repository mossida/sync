use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::devices::models::DeviceId;
use crate::integrations::classes::Class;

pub type EntityId = Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityAttributes {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub enabled: bool,
    pub available: bool,
    pub class: Class,
    pub attributes: EntityAttributes,
    pub device: DeviceId,
}

pub trait EntityFactory {
    fn build_entity(device_id: DeviceId) -> Entity;
}
