use serde::Serialize;
use surrealdb::sql::Thing;

#[derive(Serialize)]
pub struct EventType {
    pub name: String,
    pub entity_type: Thing,
}
