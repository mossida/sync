use async_trait::async_trait;
use ractor::Actor;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Priority {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub id: Thing,
    pub reference: String,
    pub configuration: Value,
    pub priority: Priority,
}

#[async_trait]
pub trait ComponentManager: Actor + Send + Sync {}

pub mod api;
pub mod classes;
pub mod components;
pub mod helpers;
