use async_trait::async_trait;
use ractor::concurrency::JoinHandle;
use ractor::{Actor, ActorCell, ActorRef, SpawnErr};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use surreal_id::NewId;
use surrealdb::opt::RecordId;
use surrealdb::sql::Id;

use crate::integrations::components::Integration;

#[derive(Debug, Deserialize_repr, Serialize_repr, Clone)]
#[repr(u8)]
pub enum Priority {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ComponentId(RecordId);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Component {
    pub id: ComponentId,
    pub reference: Integration,
    pub configuration: Value,
    pub priority: Priority,
}

impl NewId for ComponentId {
    const TABLE: &'static str = "component";

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        ComponentId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }

    fn get_inner_string(&self) -> String {
        self.0.to_string()
    }
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
        Actor::spawn_linked(Some(self.id.id_without_brackets()), T::new(), args, supervisor).await
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
