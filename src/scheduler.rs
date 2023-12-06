use async_trait::async_trait;
use hashbrown::HashMap;
use log::error;
use ractor::{Actor, ActorProcessingErr, ActorRef, SupervisionEvent};

use crate::integrations;

pub enum AdapterMessage {
    Update, // Triggers forced update
}

pub struct Scheduler {}

pub struct SchedulerState {
    adapters: HashMap<String, ActorRef<AdapterMessage>>,
}

pub enum SchedulerMessage {
    Register, // Spawns a new adapter
}

#[async_trait]
impl Actor for Scheduler {
    type Msg = ();
    type State = SchedulerState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(SchedulerState {
            adapters: Default::default(),
        })
    }

    async fn post_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // Fetch all existing components from database
        let _components = integrations::api::list_all().await;

        /*for component in components {
            Actor::spawn_linked(Some(component.id.to_string())).await;
        }*/

        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            _ => {}
        }

        Ok(())
    }

    async fn handle_supervisor_evt(
        &self,
        _: ActorRef<Self::Msg>,
        message: SupervisionEvent,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            SupervisionEvent::ActorPanicked(cell, error) => {
                // TODO: Report incident to user
                error!(
                    "Integration adapter ({}) panicked with error {}",
                    cell.get_name().unwrap_or(cell.get_id().to_string()),
                    error.to_string()
                )
            }
            _ => {}
        }

        Ok(())
    }
}
