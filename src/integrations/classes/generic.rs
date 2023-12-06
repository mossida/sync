use async_trait::async_trait;

use crate::entities::models::EntityId;

#[async_trait]
pub trait Generic<T, U> {
    fn entity(&self) -> EntityId;
    async fn attributes(&self) -> T;
    async fn data(&self) -> U;
}
