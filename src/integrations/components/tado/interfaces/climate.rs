use async_trait::async_trait;
use ractor::{Actor, ActorProcessingErr, ActorRef};

use crate::entities::models::EntityId;
use crate::integrations::components::tado::client::Client;

pub struct ClimateInterface {}

pub struct Base {
    pub client: Client,
    pub entity: EntityId, // Which enity this climate interface is managing
}

#[async_trait]
impl Actor for ClimateInterface {
    type Msg = ();
    type State = Base;
    type Arguments = Base;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(args)
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        _message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // Push state to database

        todo!()
    }
}
