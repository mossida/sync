use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: Option<Thing>,
    pub name: String,
    pub r#type: String,
    pub entity: Thing,
    pub fired_at: Datetime,
}
