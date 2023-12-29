use log::error;
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef, SupervisionEvent};

pub struct Spawner;

#[async_trait]
impl Actor for Spawner {
    type Msg = ();
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(args)
    }

    async fn handle_supervisor_evt(
        &self,
        _: ActorRef<Self::Msg>,
        message: SupervisionEvent,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            SupervisionEvent::ActorStarted(_) => {}
            SupervisionEvent::ActorTerminated(_, _, _) => {}
            SupervisionEvent::ActorPanicked(_, err) => {
                error!("Actor panicked: {:?}", err);
            }
            SupervisionEvent::ProcessGroupChanged(_) => {}
            _ => {}
        }

        Ok(())
    }
}
