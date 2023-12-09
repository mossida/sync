use async_trait::async_trait;
use log::info;
use ractor::concurrency::Duration;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use surreal_id::NewId;
use surrealdb::sql::Id;

use crate::devices::models::DeviceId;
use crate::entities::models::{Entity, EntityId, Updates};
use crate::integrations::classes::climate::{Climate, Preset};
use crate::integrations::classes::Class;
use crate::integrations::components::tado::client::Client;
use crate::integrations::components::tado::data::zone::Zone;
use crate::scheduler::definitions::{InterfaceMessage, SchedulerMessage};
use crate::{entities, scheduler};

pub struct ClimateInterface {
    pub zone: Zone,
}

pub struct State {
    pub client: Client,
    pub entity_id: EntityId,
}

#[derive(Debug)]
pub struct Arguments {
    pub client: Client,
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
        let device = self.zone.devices.get(0).unwrap();

        let entities = entities::api::get_by_device::<Preset>(
            DeviceId::new(device.serial_no.clone()).unwrap(),
        )
        .await?;

        let entity = entities.iter().find(|e| e.class == Class::Climate);

        Ok(match entity {
            None => {
                let id = EntityId::new(Id::rand().to_string()).unwrap();
                let _ = entities::api::create(
                    Entity {
                        id: id.clone(),
                        state: Preset::Unknown,
                        enabled: true,
                        available: false,
                        class: Class::Climate,
                        attributes: Default::default(),
                    },
                    DeviceId::new(device.serial_no.clone()).unwrap(),
                    Updates {
                        with_polling: true,
                        polling_interval: 5,
                    },
                )
                .await?;

                State {
                    client: args.client,
                    entity_id: id,
                }
            }
            Some(_) => State {
                client: args.client,
                entity_id: entity.unwrap().id.clone(),
            },
        })
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        info!(
            "Starting climate interface for {}",
            state.entity_id.get_inner_string()
        );

        scheduler::get()
            .send_message(SchedulerMessage::RequestPolling(
                Duration::from_secs(5),
                myself,
            ))
            .unwrap();

        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            InterfaceMessage::Update => {
                let data = state.client.get_zone_state(&self.zone).await?;
                entities::api::set_state(&state.entity_id, data.tado_mode).await?;
            }
        }

        Ok(())
    }
}

impl Climate for ClimateInterface {}
