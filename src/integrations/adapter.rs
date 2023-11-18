use crate::integrations::interface::InterfaceManager;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type AdapterId = Uuid;

pub struct Adapter {
    pub id: AdapterId,
    pub interfaces: Vec<Arc<Mutex<dyn InterfaceManager>>>,
}

#[async_trait]
pub trait AdapterManager: Send + Sync {
    // To be decided which functions
    async fn setup(&self);
}
