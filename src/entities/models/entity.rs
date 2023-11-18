use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub type EntityId = Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    id: Option<EntityId>,
    name: String,
}
