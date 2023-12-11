use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use surrealdb::opt::RecordId;
use surrealdb::sql::{Datetime, Id};

use crate::entities::models::EntityId;
use crate::integrations::classes::Attributes;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateId(RecordId);

impl NewId for StateId {
    const TABLE: &'static str = "state";

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        StateId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }

    fn get_inner_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub id: StateId,
    pub state: String,
    pub attributes: Attributes,
    pub updated_at: Datetime,
    pub entity_id: EntityId,
}

pub trait StateFactory {
    type State;
    type Attributes;
}
