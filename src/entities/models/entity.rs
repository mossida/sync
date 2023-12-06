use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::devices::models::DeviceId;
use crate::integrations::classes::Class;

pub type EntityId = Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityAttributes {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,
    name: String,
    enabled: bool,
    available: bool,
    class: Class,
    attributes: EntityAttributes,
    device: DeviceId,
}

pub trait EntityFactory {
    fn build(&self) -> Entity;
}
