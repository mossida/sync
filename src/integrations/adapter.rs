use async_trait::async_trait;
use uuid::Uuid;

pub type AdapterId = Uuid;

#[async_trait]
#[typetag::serde(tag = "type", content = "adapter")]
pub trait AdapterManager: Send + Sync {
    fn id(&self) -> AdapterId;

    async fn setup(&mut self);

    async fn main(&self);
}
