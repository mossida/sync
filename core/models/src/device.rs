use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub serial: String,
    pub model: String,
    pub manufacturer: String,
    pub sw_version: Option<String>,
    pub hw_version: Option<String>,
}
