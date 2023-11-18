use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::models::{EntityId, EntitySchema};

pub type InterfaceId = Uuid;

pub enum InterfaceType {
    SCHEDULED = 0,
    POLLING = 1,
}

// This is the base that every interface should include
pub struct Interface {
    pub id: InterfaceId,
    pub kind: InterfaceType,
    pub domain: EntitySchema,
    pub entities: Vec<EntityId>,
}

#[async_trait]
pub trait InterfaceManager: Send + Sync {
    async fn setup(&mut self);
    // Called by Sync when the interface is registered
    async fn update(&mut self); // Called by the Scheduler when update of data is necessary

    async fn execute_action(&self); // Called by Sync when an action is triggered
}
