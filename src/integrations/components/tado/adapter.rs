use async_trait::async_trait;
use hashbrown::HashMap;
use log::error;
use ractor::{Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use surreal_id::NewId;
use surrealdb::sql::Id;

use crate::devices::models::{Device, DeviceId};
use crate::integrations::components::tado::client::Client;
use crate::integrations::components::tado::data::user::User;
use crate::integrations::components::tado::interfaces::climate::{Arguments, ClimateInterface};
use crate::integrations::{Component, ComponentManager};
use crate::scheduler::definitions::{AdapterMessage, InterfaceMessage, SchedulerMessage};
use crate::{devices, scheduler};

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
        let configuration = serde_json::from_value(args.configuration.clone()).unwrap();
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
            let (cell, handle) = Actor::spawn_linked(
                Some(format!(
                    "{}/{}/climate",
                    myself.get_name().unwrap(),
                    Id::rand()
                )),
                ClimateInterface { zone },
                Arguments {
                    client: state.client.clone(),
                },
                myself.get_cell(),
            )
            .await?;

            state
                .interfaces
                .insert(cell.get_name().unwrap().to_string(), cell);
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
