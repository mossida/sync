use async_trait::async_trait;
use ractor::{Actor, ActorProcessingErr, ActorRef};

use crate::devices::models::DeviceId;
use crate::entities;
use crate::entities::models::{EntityFactory, EntityId};
use crate::integrations::classes::climate::Climate;
use crate::integrations::components::tado::client::Client;
use crate::scheduler::InterfaceMessage;
use crate::states::models::state::StateFactory;

pub struct ClimateInterface {}

pub struct State {
    pub client: Client,
    pub entity_id: EntityId,
}

pub struct Arguments {
    pub client: Client,
    pub device_id: DeviceId,
    pub entity_id: Option<EntityId>,
}

#[async_trait]
impl Actor for ClimateInterface {
    type Msg = InterfaceMessage;
    type State = State;
    type Arguments = Arguments;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        if let Some(id) = args.entity_id {
            Ok(State {
                client: args.client,
                entity_id: id,
            })
        } else {
            let entity =
                entities::api::create(ClimateInterface::build_entity(args.device_id)).await?;

            Ok(State {
                client: args.client,
                entity_id: entity[0].id.clone(),
            })
        }
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

impl Climate for ClimateInterface {}
