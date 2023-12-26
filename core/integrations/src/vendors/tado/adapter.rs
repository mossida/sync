#![allow(unused_variables)]

use ractor::{async_trait, ActorProcessingErr, ActorRef};

use crate::scheduler::models::AdapterMessage;
use crate::vendors::tado::client;

pub struct Tado;

pub struct State {
    pub user: client::data::user::User,
    pub client: client::Client,
}

#[async_trait]
impl ractor::Actor for Tado {
    type Msg = AdapterMessage;
    type State = State;
    type Arguments = serde_json::Value;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let configuration = serde_json::from_value(args)?;
        let mut client = client::Client::new(configuration).await?;
        let user = client.get_me().await?;
        client.use_home(user.homes[0].id);

        Ok(State { client, user })
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        todo!()
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        todo!()
    }
}
