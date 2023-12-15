use async_trait::async_trait;
use log::info;
use ractor::concurrency::Duration;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use surreal_id::NewId;
use surrealdb::sql::{Datetime, Id};

use crate::devices::models::DeviceId;
use crate::entities::models::{Entity, EntityId, State as EntityState, Updates};
use crate::integrations::classes::climate::{Attribute, Climate, HVACAction, HVACMode};
use crate::integrations::classes::Class;
use crate::integrations::components::tado::client::Client;
use crate::integrations::components::tado::data::capability::CapabilityType;
use crate::integrations::components::tado::data::states::{Action, Mode};
use crate::integrations::components::tado::data::zone::Zone;
use crate::scheduler::definitions::{InterfaceMessage, SchedulerMessage};
use crate::{entities, scheduler};

pub struct ClimateInterface {
    pub zone: Zone,
}

pub struct State {
    pub client: Client,
    pub entity: Entity,
}

#[derive(Debug)]
pub struct Arguments {
    pub client: Client,
    pub entity: Option<Entity>,
}

#[async_trait]
impl Actor for ClimateInterface {
    type Msg = InterfaceMessage;
    type State = State;
    type Arguments = Arguments;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let device = self.zone.devices.get(0).unwrap();

        Ok(match args.entity {
            None => {
                let entity = entities::api::create(
                    vec![Entity {
                        id: EntityId::new(Id::rand().to_string()).unwrap(),
                        enabled: true,
                        available: false,
                        class: Class::Climate,
                        attributes: Default::default(),
                        interface: myself.get_name().unwrap().into(),
                        state: EntityState {
                            state: serde_json::to_value::<HVACMode>(HVACMode::Off).unwrap(),
                            attributes: Default::default(),
                            updated_at: Datetime::from(chrono::Utc::now()),
                        },
                    }],
                    DeviceId::new(device.serial_no.clone()).unwrap(),
                    Updates {
                        with_polling: true,
                        polling_interval: 5,
                    },
                )
                .await?;

                State {
                    client: args.client,
                    entity: entity[0].clone(),
                }
            }
            Some(entity) => State {
                client: args.client,
                entity,
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
            state.entity.id.get_inner_string()
        );

        let capabilities = state.client.get_capabilities(&self.zone).await?;

        if capabilities.r#type == CapabilityType::Heating {
            state.entity.state.attributes.merge_vec(vec![
                Attribute::MaxHumidity(100.0),
                Attribute::MinHumidity(0.0),
                Attribute::MaxTemp(capabilities.temperatures.celsius.max as f32),
                Attribute::MinTemp(capabilities.temperatures.celsius.min as f32),
                Attribute::TargetTemperatureStep(capabilities.temperatures.celsius.step as f32),
            ]);

            state.entity =
                entities::api::merge_attributes(&state.entity.id, &state.entity.state.attributes)
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

                state.entity.state.attributes.merge_vec(vec![
                    Attribute::CurrentTemperature(
                        data.sensor_data_points.inside_temperature.celsius as f32,
                    ),
                    Attribute::CurrentHumidity(data.sensor_data_points.humidity.percentage as f32),
                    Attribute::HvacMode(hvac_mode.clone()),
                    Attribute::HvacAction(hvac_action),
                    Attribute::Preset(data.tado_mode.into()),
                ]);

                state.entity = entities::api::merge_attributes(
                    &state.entity.id,
                    &state.entity.state.attributes,
                )
                .await?
                .unwrap();

                state.entity = entities::api::set_state(
                    &state.entity.id,
                    serde_json::to_value::<HVACMode>(hvac_mode)?,
                )
                .await?
                .unwrap();
            }
        }

        Ok(())
    }
}

impl Climate for ClimateInterface {}
