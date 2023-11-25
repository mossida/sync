use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::integrations::interface::{InterfaceBase, InterfaceManager};

#[derive(Serialize, Deserialize)]
pub struct ExampleInterfaceData {
    pub base: InterfaceBase,
}

#[async_trait]
#[typetag::serde]
impl InterfaceManager for ExampleInterfaceData {
    fn base(&self) -> &InterfaceBase {
        &self.base
    }

    async fn setup(&mut self) {
        println!("setup called");
    }

    async fn update(&mut self) {
        println!("update called");
    }

    async fn execute_action(&self) {
        todo!()
    }
}
