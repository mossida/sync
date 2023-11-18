use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::integrations::interface::{Interface, InterfaceManager};

#[derive(Serialize, Deserialize)]
pub struct ExampleInterface {
    pub base: Interface,
}

#[async_trait]
#[typetag::serde]
impl InterfaceManager for ExampleInterface {
    fn base(&self) -> &Interface {
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
