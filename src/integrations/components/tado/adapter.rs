use async_trait::async_trait;
use ractor::{Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use serde_json::Value;
use surrealdb::sql::Thing;

use crate::devices;
use crate::devices::models::Device;
use crate::integrations::components::tado::client::Client;
use crate::integrations::components::tado::data::user::User;
use crate::integrations::ComponentManager;
use crate::scheduler::AdapterMessage;

pub struct Adapter;

pub struct State {
    pub client: Client,
    pub user: User,
}

#[async_trait]
impl Actor for Adapter {
    type Msg = AdapterMessage;
    type State = State;
    type Arguments = Value;

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        // Init http client
        let configuration = serde_json::from_value(args).unwrap();
        let mut client = Client::new(configuration).await?;
        let user = client.get_me().await?;
        client.use_home(user.homes[0].id);

        Ok(State { client, user })
    }

    async fn post_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let devices = state.client.get_devices().await?;

        for device in devices {
            let _ = devices::api::create(Device {
                id: Thing::from(("device", device.serial_no.as_str())),
                name: device.device_type,
                serial: device.short_serial_no,
                model: "".to_string(),
                manufacturer: "tado".to_string(),
                sw_version: device.current_fw_version,
                hw_version: "1.0".to_string(),
                entities: vec![],
            })
            .await;
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
        _message: SupervisionEvent,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        todo!()
    }
}

impl ComponentManager for Adapter {
    fn new() -> Self {
        Adapter {}
    }
}
