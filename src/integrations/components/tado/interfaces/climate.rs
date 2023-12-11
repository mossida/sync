use async_trait::async_trait;
use log::info;
use ractor::concurrency::Duration;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use surreal_id::NewId;
use surrealdb::sql::{Datetime, Id};

use crate::devices::models::DeviceId;
use crate::entities::models::{Entity, EntityId, Updates};
use crate::integrations::classes::climate::{Attribute, Climate, HVACAction, HVACMode, Preset};
use crate::integrations::classes::Class;
use crate::integrations::components::tado::client::Client;
use crate::integrations::components::tado::data::capability::CapabilityType;
use crate::integrations::components::tado::data::states::{Action, Mode};
use crate::integrations::components::tado::data::zone::Zone;
use crate::scheduler::definitions::{InterfaceMessage, SchedulerMessage};
use crate::states::models::state::StateId;
use crate::{entities, scheduler, states};

pub struct ClimateInterface {
    pub zone: Zone,
}

pub struct State {
    pub client: Client,
    pub entity: Entity,
    pub state: states::models::state::State,
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

        let entities =
            entities::api::get_by_device(DeviceId::new(device.serial_no.clone()).unwrap()).await?;

        // FIXME: Wrong, a device could have multiple climate entities
        let exist_entity = entities.iter().find(|e| e.class == Class::Climate);

        Ok(match exist_entity {
            None => {
                let id = EntityId::new(Id::rand().to_string()).unwrap();

                let entity = entities::api::create(
                    Entity {
                        id: id.clone(),
                        enabled: true,
                        available: false,
                        class: Class::Climate,
                        attributes: Default::default(),
                        state_id: None,
                    },
                    DeviceId::new(device.serial_no.clone()).unwrap(),
                    Updates {
                        with_polling: true,
                        polling_interval: 5,
                    },
                )
                .await?;

                let entity_state = states::api::set_state(states::models::state::State {
                    id: StateId::new(Id::rand().to_string()).unwrap(),
                    state: serde_json::to_string::<Preset>(&Preset::Unknown).unwrap(),
                    attributes: Default::default(),
                    updated_at: Datetime::from(chrono::Utc::now()),
                    entity_id: id.clone(),
                })
                .await?;

                State {
                    client: args.client,
                    entity: entity[0].clone(),
                    state: entity_state.clone().unwrap(),
                }
            }
            Some(entity) => {
                let state = states::api::get_state_of_entity(entity.id.clone()).await?;

                State {
                    client: args.client,
                    entity: entity.clone(),
                    state: state.unwrap(),
                }
            }
        })
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        info!(
            "Starting climate interface for {}",
            state.entity.id.get_inner_string()
        );

        let capabilities = state.client.get_capabilities(&self.zone).await?;

        if capabilities.r#type == CapabilityType::Heating {
            state.state.attributes.merge_vec(vec![
                Attribute::MaxTemp(capabilities.temperatures.celsius.max as f32),
                Attribute::MinTemp(capabilities.temperatures.celsius.min as f32),
                Attribute::TargetTemperatureStep(capabilities.temperatures.celsius.step as f32),
            ]);

            state.state =
                states::api::set_attributes(state.state.id.clone(), state.state.attributes.clone())
                    .await?
                    .unwrap();
        }

        scheduler::get()
            .send_message(SchedulerMessage::RequestPolling(
                Duration::from_secs(15),
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

                //let target_temperature = data.setting.map_or(0.0, |s| s.temperature.celsius);

                let hvac_action = data.activity_data_points.heating_power.map_or(
                    HVACAction::Idle,
                    |heating_power| match heating_power.percentage {
                        // FIXME: don't use numbers in matching
                        0.0 => HVACAction::Idle,
                        _ => HVACAction::Heating,
                    },
                );

                let hvac_mode = data.overlay.map_or(Mode::SmartSchedule.into(), |_| {
                    data.setting.map_or(HVACMode::Off, |s| {
                        s.r#type.unwrap_or(s.mode.unwrap_or(Action::Off)).into()
                    })
                });

                state.state.attributes.merge_vec(vec![
                    Attribute::CurrentTemperature(
                        data.sensor_data_points.inside_temperature.celsius as f32,
                    ),
                    Attribute::CurrentHumidity(data.sensor_data_points.humidity.percentage as f32),
                    Attribute::HvacMode(hvac_mode),
                    Attribute::HvacAction(hvac_action),
                    Attribute::Preset(data.tado_mode.into()),
                    //Attribute::TargetTemperatureHigh(target_temperature as f32),
                    //Attribute::TargetTemperatureLow(target_temperature as f32),
                ]);

                states::api::set_attributes(state.state.id.clone(), state.state.attributes.clone())
                    .await?;
            }
        }

        Ok(())
    }
}

impl Climate for ClimateInterface {}
