use ractor::{Actor, ActorProcessingErr, ActorRef, GroupName};

use crate::api::rejections::{Rejection, RejectionCode};
use crate::ws::models::{MessageHandler, Sender};
use crate::ws::reply::error;

pub const GROUP_NAME: &str = "websocket_clients";

pub struct ClientActor;

pub struct ClientState {
    pub last_id: u16,
    pub sender: Sender,
}

#[async_trait::async_trait]
impl Actor for ClientActor {
    type Msg = Box<dyn MessageHandler>;
    type State = ClientState;
    type Arguments = Sender;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(ClientState {
            last_id: 0,
            sender: args,
        })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if message.id() <= state.last_id {
            error(
                message.id(),
                &state.sender,
                Rejection {
                    reason: RejectionCode::INTERFACE,
                    message: "Message has not incremental id".to_string(),
                },
            );
        } else {
            message.handle(&state.sender).await;
            state.last_id = message.id();
        }

        Ok(())
    }
}
