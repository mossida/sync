use async_trait::async_trait;
use log::debug;
use ractor::concurrency::Duration;
use ractor::{Actor, ActorProcessingErr, ActorRef};

use models::attributes::Attributes;
use models::entities::{Entity, EntityState};

use crate::classes::climate::{Attribute, HVACAction, HVACMode};
use crate::scheduler;
use crate::scheduler::models::{InterfaceMessage, SchedulerMessage};
use crate::vendors::tado::client::data::capability::CapabilityType;
use crate::vendors::tado::client::data::states::{Action, Mode};
use crate::vendors::tado::client::data::zone::Zone;
use crate::vendors::tado::client::Client;

pub struct ClimateInterface {
    pub zone: Zone,
}

pub struct State {
    pub client: Client,
    pub entity: Entity,
}

#[async_trait]
impl Actor for ClimateInterface {
    type Msg = InterfaceMessage;
    type State = State;
    type Arguments = State;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(args)
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let entity: &mut Entity = &mut state.entity;
        let capabilities = state.client.get_capabilities(&self.zone).await?;

        if capabilities.r#type == CapabilityType::Heating {
            entity
                .merge(EntityState {
                    state: entity.status.state.clone(),
                    attributes: Attributes::from(vec![
                        Attribute::MaxHumidity(100.0),
                        Attribute::MinHumidity(0.0),
                        Attribute::MaxTemp(capabilities.temperatures.celsius.max as f32),
                        Attribute::MinTemp(capabilities.temperatures.celsius.min as f32),
                        Attribute::TargetTemperatureStep(
                            capabilities.temperatures.celsius.step as f32,
                        ),
                    ]),
                    updated_at: Default::default(),
                })
                .await?;
        }

        scheduler::send(SchedulerMessage::PollInterface(
            Duration::from_secs(15),
            myself,
        ))
        .await?;

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
                debug!("Update received");
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

                state
                    .entity
                    .merge(EntityState {
                        state: hvac_mode.clone().to_string(),
                        attributes: Attributes::from(vec![
                            Attribute::CurrentTemperature(
                                data.sensor_data_points.inside_temperature.celsius as f32,
                            ),
                            Attribute::CurrentHumidity(
                                data.sensor_data_points.humidity.percentage as f32,
                            ),
                            Attribute::HvacMode(hvac_mode.clone()),
                            Attribute::HvacAction(hvac_action),
                            Attribute::Preset(data.tado_mode.into()),
                        ]),
                        updated_at: Default::default(),
                    })
                    .await?;
            }
            _ => {}
        }

        Ok(())
    }
}
