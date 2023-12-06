use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::{Id, Thing};

use crate::entities::models::EntityId;
use crate::integrations::classes::generic::Generic;

pub type StateId = Thing;

#[derive(Serialize, Deserialize)]
pub struct State<Data = Value, Attributes = Value> {
    pub id: StateId,
    pub entity: EntityId,
    pub attributes: Attributes,
    pub state: Data,
    pub updated: DateTime<Utc>,
}

#[async_trait]
pub trait StateFactory<'a>: Generic<Self::Data, Self::Attributes> + Send + Sync {
    type Data: Serialize + Deserialize<'a> + Send + Sync;
    type Attributes: Serialize + Deserialize<'a> + Send + Sync;

    async fn build_state(&self) -> State<Self::Data, Self::Attributes> {
        State {
            id: Thing::from(("state", Id::rand())),
            entity: self.entity(),
            attributes: self.attributes().await,
            state: self.data().await,
            updated: Default::default(),
        }
    }
}
