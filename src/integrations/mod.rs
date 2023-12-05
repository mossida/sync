use async_trait::async_trait;
use ractor::Actor;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub id: Uuid,
    pub reference: String,
    pub configuration: Value,
    pub priority: Priority,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            reference: String::new(),
            configuration: Value::Null,
            priority: Priority::Low,
        }
    }
}

#[async_trait]
pub trait ComponentManager: Actor + Send + Sync {}

pub mod api;
pub mod components;
pub mod helpers;
