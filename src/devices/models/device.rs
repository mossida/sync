use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing};

pub type DeviceId = Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: DeviceId,
    pub name: String,
    pub serial: String,
    pub model: String,
    pub manufacturer: String,
    pub sw_version: String,
    pub hw_version: String,
    pub entities: Vec<Thing>,
}
