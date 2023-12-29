use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub const RESOURCE: &str = "entity";

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub id: Thing,
    pub enabled: bool,
    pub available: bool,
    pub class: String,
    pub attributes: Option<serde_json::Value>,
    pub state: EntityState,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EntityState {
    pub state: serde_json::Value,
    pub attributes: Option<serde_json::Value>,
    pub updated_at: surrealdb::sql::Datetime,
}
