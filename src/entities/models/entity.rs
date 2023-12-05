use serde::{Deserialize, Serialize};
use surrealdb::sql::Uuid;

use crate::devices::models::DeviceId;

pub type EntityId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,
    name: String,
    enabled: bool,
    available: bool,
    needs_polling: bool,
    domain: String,
    device: DeviceId,
}