use async_trait::async_trait;
use ractor::{Actor, ActorProcessingErr, ActorRef};

pub struct Interface {}

pub struct State {}

#[async_trait]
impl Actor for Interface {
    type Msg = ();
    type State = State;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(State {})
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        _message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        todo!()
    }
}
