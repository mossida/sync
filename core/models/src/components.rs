use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub vendor: String,
    pub configuration: serde_json::Value,
    pub priority: u8,
}
