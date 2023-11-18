use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::models::{EntityId, EntitySchema};

pub type InterfaceId = Uuid;

#[derive(Serialize, Deserialize)]
pub enum InterfaceType {
    SCHEDULED = 0,
    POLLING = 1,
}

// This is the base that every interface should include
#[derive(Serialize, Deserialize)]
pub struct Interface {
    pub id: InterfaceId,
    pub kind: InterfaceType,
    pub domain: EntitySchema,
    pub entities: Vec<EntityId>,
}

#[async_trait]
#[typetag::serde(tag = "type", content = "interface")]
pub trait InterfaceManager: Send + Sync {
    fn base(&self) -> &Interface;

    // Called by Sync when the interface is registered
    async fn setup(&mut self);

    // Called by the Scheduler when update of data is necessary
    async fn update(&mut self);

    // Called by Sync when an action is triggered
    async fn execute_action(&self);
}
