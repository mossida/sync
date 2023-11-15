use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: Option<Thing>,
    pub name: String,
    //pub data: String,
    //pub event_type: Thing,
    //pub entity: Thing,
    //pub fired_at: Datetime,
    //pub created_at: Datetime,
}
