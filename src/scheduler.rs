use async_trait::async_trait;
use hashbrown::HashMap;
use log::error;
use ractor::concurrency::JoinHandle;
use ractor::{Actor, ActorProcessingErr, ActorRef, SpawnErr, SupervisionEvent};
use surreal_id::NewId;

use crate::integrations;

pub enum AdapterMessage {
    Update,
    // Triggers forced update
    Action(String), // Calls an action on the adapter,
}

pub enum InterfaceMessage {
    Update,
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
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // Fetch all existing components from database
        let components = integrations::api::list_all().await;

        for component in components.unwrap() {
            let handle = component
                .reference
                .spawn(component.clone(), myself.get_cell())
                .await?;

            state
                .adapters
                .entry(component.id.id_without_brackets())
                .or_insert_with(|| handle.0);
        }

        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        _message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        Ok(())
    }

    async fn handle_supervisor_evt(
        &self,
        _: ActorRef<Self::Msg>,
        message: SupervisionEvent,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SupervisionEvent::ActorPanicked(cell, error) = message {
            // TODO: Report incident to user
            error!(
                "Integration adapter ({}) panicked with: {}",
                cell.get_name().unwrap_or(cell.get_id().to_string()),
                error.to_string()
            );
        }

        Ok(())
    }
}

impl Scheduler {
    pub async fn start() -> Result<(ActorRef<()>, JoinHandle<()>), SpawnErr> {
        Actor::spawn(Some("scheduler".to_string()), Scheduler {}, ()).await
    }
}
