use serde::{Deserialize, Serialize};
use serde_json::Value;
use surreal_id::NewId;
use surrealdb::opt::RecordId;
use surrealdb::sql::{Datetime, Id};

use crate::integrations::classes::{Attributes, Class};
use crate::scheduler::definitions::InterfaceName;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityId(RecordId);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub enabled: bool,
    pub available: bool,
    pub class: Class,
    pub attributes: Option<Value>,
    pub interface: InterfaceName,
    pub state: State,
}

impl NewId for EntityId {
    const TABLE: &'static str = "entity";

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        EntityId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }

    fn get_inner_string(&self) -> String {
        self.0.to_string()
    }
}

pub trait EntityFactory {
    fn build_entity() -> Entity;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub state: Value,
    pub attributes: Attributes,
    pub updated_at: Datetime,
}

pub trait StateFactory {
    type State;
    type Attributes;
}
