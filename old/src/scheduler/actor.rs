use async_trait::async_trait;
use log::{debug, error, info, warn};
use ractor::{Actor, ActorProcessingErr, ActorRef, SupervisionEvent};

use crate::integrations;
use crate::scheduler::definitions::{
    InterfaceMessage, Scheduler, SchedulerMessage, SchedulerState,
};

#[async_trait]
impl Actor for Scheduler {
    type Msg = SchedulerMessage;
    type State = SchedulerState;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(SchedulerState::new())
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // Fetch all existing components from database
        let components = integrations::api::list_all().await;

        for component in components.unwrap() {
            let _ = component
                .reference
                .spawn(component.clone(), myself.get_cell())
                .await?;
        }

        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            SchedulerMessage::RequestPolling(interval, cell) => {
                debug!("Scheduler received polling request for {}", cell.get_id());
                state.add_poller(
                    cell.get_id().to_string(),
                    cell.send_interval(interval, || InterfaceMessage::Update),
                )
            }
            SchedulerMessage::StopPolling(cell) => {
                debug!(
                    "Scheduler received polling stop request for {}",
                    cell.get_id()
                );
                let handle = state.remove_poller(cell.get_id().to_string());
                if handle.is_none() {
                    warn!("Cannot stop poller, the interface is not polling.")
                } else {
                    handle.unwrap().abort()
                }
            }
            SchedulerMessage::Ping => {
                info!("Scheduler pong");
            }
        }

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
