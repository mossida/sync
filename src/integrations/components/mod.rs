use ractor::concurrency::JoinHandle;
use ractor::{Actor, ActorCell, ActorRef, SpawnErr};
use serde::{Deserialize, Serialize};

use crate::integrations::Component;
use crate::scheduler::AdapterMessage;

pub mod tado;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Integration {
    Tado,
}

impl Integration {
    pub async fn spawn(
        &self,
        component: &Component,
        supervisor: ActorCell,
    ) -> Result<(ActorRef<AdapterMessage>, JoinHandle<()>), SpawnErr> {
        match self {
            Integration::Tado => {
                component
                    .build::<tado::adapter::Adapter>(supervisor, component.configuration.clone())
                    .await
            }
        }
    }
}
