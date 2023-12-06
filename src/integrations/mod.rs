use async_trait::async_trait;
use ractor::concurrency::JoinHandle;
use ractor::{Actor, ActorCell, ActorRef, SpawnErr};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use surrealdb::sql::Thing;
use crate::integrations::components::Integration;

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
    pub reference: Integration,
    pub configuration: Value,
    pub priority: Priority,
}

impl Component {
    pub async fn build<T>(
        &self,
        supervisor: ActorCell,
        args: T::Arguments,
    ) -> Result<(ActorRef<T::Msg>, JoinHandle<()>), SpawnErr>
    where
        T: ComponentManager,
    {
        Actor::spawn_linked(Some(self.id.to_string()), T::new(), args, supervisor).await
    }
}

#[async_trait]
pub trait ComponentManager: Actor + Send + Sync {
    fn new() -> Self;
}

pub mod api;
pub mod classes;
pub mod components;
pub mod helpers;
