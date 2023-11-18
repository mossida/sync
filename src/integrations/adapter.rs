use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::integrations::interface::{InterfaceId, InterfaceManager};
use crate::types::SyncMap;

pub type AdapterId = Uuid;

#[derive(Serialize, Deserialize)]
pub struct Adapter {
    pub id: AdapterId,

    /** Not interested in serialization since the interfaces are
    created manually */
    #[serde(skip)]
    pub interfaces: SyncMap<InterfaceId, dyn InterfaceManager>,
}

#[async_trait]
#[typetag::serde(tag = "type", content = "adapter")]
pub trait AdapterManager: Send + Sync {
    fn base(&self) -> &Adapter;

    async fn setup(&mut self);

    async fn main(&self);
}
