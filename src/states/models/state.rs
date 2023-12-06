use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::entities::models::EntityId;

pub type StateId = Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct State<T> {
    pub id: StateId,
    pub entity_id: EntityId,
    pub state: String,
    pub attributes: Vec<T>,
    pub last_updated: DateTime<Utc>,
}

pub trait StateFactory {
    type State;
    type Attributes;
}
