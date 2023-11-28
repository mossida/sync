use serde::{Deserialize, Serialize};
use surrealdb::sql::Uuid;

pub type DeviceId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    id: DeviceId,
    name: String,
    serial: String,
    model: String,
    manufacturer: String,
    sw_version: String,
    hw_version: String,
    entities: Vec<Uuid>,
}
