use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::integrations::adapter::{AdapterId, AdapterManager};

#[derive(Serialize, Deserialize)]
pub struct ExampleAdapter {
    id: AdapterId,
}

#[async_trait]
#[typetag::serde]
impl AdapterManager for ExampleAdapter {
    fn id(&self) -> AdapterId {
        self.id
    }

    async fn setup(&mut self) {}

    async fn main(&self) {
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
