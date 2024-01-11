use async_trait::async_trait;
use log::debug;
use ractor::concurrency::Duration;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use uom::si::f64::{Ratio, TemperatureInterval, ThermodynamicTemperature};
use uom::si::ratio::percent;
use uom::si::temperature_interval;
use uom::si::thermodynamic_temperature::degree_celsius;

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
        _: ActorRef<Self::Msg>,
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
                        Attribute::MaxHumidity(Ratio::new::<percent>(100.0)),
                        Attribute::MinHumidity(Ratio::new::<percent>(0.0)),
                        Attribute::MaxTemp(ThermodynamicTemperature::new::<degree_celsius>(
                            capabilities.temperatures.celsius.max,
                        )),
                        Attribute::MinTemp(ThermodynamicTemperature::new::<degree_celsius>(
                            capabilities.temperatures.celsius.min,
                        )),
                        Attribute::TargetTemperatureStep(TemperatureInterval::new::<
                            temperature_interval::degree_celsius,
                        >(
                            capabilities.temperatures.celsius.step
                        )),
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

                let heating_power = data
                    .activity_data_points
                    .heating_power
                    .map_or(0.0, |heating_power| heating_power.percentage);

                let hvac_action = if heating_power > 0.0 {
                    HVACAction::Heating
                } else {
                    HVACAction::Idle
                };

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
                            Attribute::CurrentTemperature(ThermodynamicTemperature::new::<
                                degree_celsius,
                            >(
                                data.sensor_data_points.inside_temperature.celsius,
                            )),
                            Attribute::CurrentHumidity(Ratio::new::<percent>(
                                data.sensor_data_points.humidity.percentage,
                            )),
                            Attribute::HvacMode(hvac_mode.clone()),
                            Attribute::HvacAction(hvac_action),
                            Attribute::Preset(data.tado_mode.into()),
                            Attribute::HvacPower(Ratio::new::<percent>(heating_power)),
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
