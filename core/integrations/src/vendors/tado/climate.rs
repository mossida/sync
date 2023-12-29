use async_trait::async_trait;
use log::info;
use ractor::concurrency::Duration;
use ractor::{Actor, ActorProcessingErr, ActorRef};

use models::entity::Entity;

use crate::scheduler;
use crate::scheduler::models::{InterfaceMessage, SchedulerMessage};
use crate::vendors::tado::client::data::capability::CapabilityType;
use crate::vendors::tado::client::data::zone::Zone;
use crate::vendors::tado::client::Client;

pub struct ClimateInterface {
    pub zone: Zone,
    pub entity: Entity,
}

pub struct State {
    pub client: Client,
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
        info!(
            "Starting climate interface for {}",
            self.entity.id.id.to_string()
        );

        let capabilities = state.client.get_capabilities(&self.zone).await?;

        if capabilities.r#type == CapabilityType::Heating {
            /*state.entity.state.attributes.merge_vec(vec![
                Attribute::MaxHumidity(100.0),
                Attribute::MinHumidity(0.0),
                Attribute::MaxTemp(capabilities.temperatures.celsius.max as f32),
                Attribute::MinTemp(capabilities.temperatures.celsius.min as f32),
                Attribute::TargetTemperatureStep(capabilities.temperatures.celsius.step as f32),
            ]);

            state.entity =
                entities::api::merge_attributes(&state.entity.id, &state.entity.state.attributes)
                    .await?
                    .unwrap();*/
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
                let data = state.client.get_zone_state(&self.zone).await?;

                dbg!(data);

                /*let hvac_action = data.activity_data_points.heating_power.map_or(
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
                .unwrap();*/
            }
            _ => {}
        }

        Ok(())
    }
}
