use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::models::EntitySchema;
use crate::helpers::Helper;
use crate::integrations::adapter::{Adapter, AdapterManager};
use crate::integrations::adapters::example::interface::ExampleInterface;
use crate::integrations::interface::{Interface, InterfaceType};
use crate::scheduler::Scheduler;

#[derive(Serialize, Deserialize)]
pub struct ExampleAdapter {
    pub base: Adapter,
}

#[async_trait]
#[typetag::serde]
impl AdapterManager for ExampleAdapter {
    fn base(&self) -> &Adapter {
        &self.base
    }

    async fn setup(&mut self) {
        let id = Uuid::new_v4();
        let integration = Helper::create_sync_object(ExampleInterface {
            base: Interface {
                id,
                kind: InterfaceType::SCHEDULED,
                domain: EntitySchema {},
                entities: vec![],
            },
        });

        self.base.interfaces.insert(id, integration);
    }

    async fn main(&self) {
        for (_, recipient) in &self.base.interfaces {
            Scheduler::schedule_update(recipient.clone());
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
