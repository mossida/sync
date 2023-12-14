use async_trait::async_trait;
use hashbrown::HashMap;
use log::error;
use ractor::{Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use surreal_id::NewId;

use crate::devices::models::{Device, DeviceId};
use crate::integrations::classes::Class;
use crate::integrations::components::tado::client::Client;
use crate::integrations::components::tado::data::user::User;
use crate::integrations::components::tado::interfaces::climate::{Arguments, ClimateInterface};
use crate::integrations::components::tado::models::Configuration;
use crate::integrations::{Component, ComponentManager};
use crate::scheduler::definitions::{
    AdapterMessage, InterfaceMessage, InterfaceName, SchedulerMessage,
};
use crate::{devices, entities, scheduler};

pub struct Adapter;

pub struct State {
    pub interfaces: HashMap<String, ActorRef<InterfaceMessage>>,
    pub component: Component,
    pub client: Client,
    pub user: User,
}

#[async_trait]
impl Actor for Adapter {
    type Msg = AdapterMessage;
    type State = State;
    type Arguments = Component;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        // Init http client
        let configuration: Configuration = serde_json::from_value(args.configuration.clone())?;
        let mut client = Client::new(configuration).await?;
        let user = client.get_me().await?;
        client.use_home(user.homes[0].id);

        Ok(State {
            client,
            user,
            component: args,
            interfaces: Default::default(),
        })
    }

    async fn post_start(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let tado_zones = state.client.get_zones().await?;
        let tado_devices = state.client.get_devices().await?;

        // FIXME: Find a way to not trigger error when inserting already existing devices
        let _ = devices::api::create_multiple(
            tado_devices
                .iter()
                .map(|d| Device {
                    id: DeviceId::new(d.serial_no.as_str()).unwrap(),
                    name: d.device_type.clone(),
                    serial: d.short_serial_no.clone(),
                    model: "".to_string(),
                    manufacturer: "tado".to_string(),
                    sw_version: d.current_fw_version.clone(),
                    hw_version: "1.0".to_string(),
                })
                .collect(),
            state.component.id.clone(),
        )
        .await;

        // Spawn climate interfaces
        for zone in tado_zones {
            let entities = entities::api::fetch_by_device_and_class(
                DeviceId::new(zone.devices[0].serial_no.clone()).unwrap(),
                Class::Climate,
            )
            .await?;

            let name = match entities.is_empty() {
                true => InterfaceName::new(myself.get_name().unwrap(), Class::Climate).into(),
                false => entities[0].interface.clone(),
            };

            let (cell, _) = Actor::spawn_linked(
                Some(name.clone().into()),
                ClimateInterface { zone },
                Arguments {
                    client: state.client.clone(),
                    entity: match entities.is_empty() {
                        true => None,
                        false => Some(entities[0].clone()),
                    },
                },
                myself.get_cell(),
            )
            .await?;

            state.interfaces.insert(name.into(), cell);
        }

        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        _message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        todo!()
    }

    async fn handle_supervisor_evt(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: SupervisionEvent,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            SupervisionEvent::ActorPanicked(cell, msg) => {
                error!("Interface {} panicked: {}", cell.get_name().unwrap(), msg);
                scheduler::get()
                    .send_message(SchedulerMessage::StopPolling(cell))
                    .unwrap();
            }
            _ => {}
        }

        Ok(())
    }
}

impl ComponentManager for Adapter {
    fn new() -> Self {
        Adapter {}
    }
}
