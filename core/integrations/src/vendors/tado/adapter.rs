#![allow(unused_variables)]

use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use surrealdb::sql::Thing;

use models::component::Component;
use models::device::Device;
use models::entities::Entity;
use models::{device, entities};

use crate::classes::Class;
use crate::scheduler::models::AdapterMessage;
use crate::vendors::tado::client;
use crate::vendors::tado::climate::ClimateInterface;
use crate::vendors::tado::climate::State as ClimateState;

pub struct Tado;

pub struct State {
	pub user: client::data::user::User,
	pub client: client::Client,
	pub component: Component,
}

#[async_trait]
impl ractor::Actor for Tado {
	type Msg = AdapterMessage;
	type State = State;
	type Arguments = Component;

	async fn pre_start(
		&self,
		myself: ActorRef<Self::Msg>,
		args: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		let configuration = serde_json::from_value(args.configuration.clone())?;
		let mut client = client::Client::new(configuration).await?;
		let user = client.get_me().await?;
		client.use_home(user.homes[0].id);

		Ok(State {
			client,
			user,
			component: args,
		})
	}

	async fn post_start(
		&self,
		myself: ActorRef<Self::Msg>,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		let tado_zones = state.client.get_zones().await?;
		let tado_devices = state.client.get_devices().await?;

		for device in tado_devices {
			state
				.component
				.controls(&Device {
					id: Thing {
						tb: device::RESOURCE.to_owned(),
						id: device.serial_no.into(),
					},
					name: device.device_type,
					serial: device.short_serial_no,
					model: "".to_owned(),
					manufacturer: "tado".to_owned(),
					sw_version: Some(device.current_fw_version),
					hw_version: Some("1.0".to_owned()),
				})
				.await?;
		}

		// Generate entities
		let devices = state.component.get_devices().await?;
		for zone in tado_zones {
			let device =
				devices.iter().find(|d| d.id.id.to_string() == zone.devices[0].serial_no).unwrap();

			let entity = Entity {
				id: Thing {
					tb: entities::RESOURCE.to_owned(),
					id: device.serial.clone().into(),
				},
				enabled: true,
				available: false,
				class: Class::Climate.to_string(),
				attributes: Default::default(),
				status: Default::default(),
			};

			device.updates(&entity).await?;

			// TODO: Define standard name for interfaces
			Actor::spawn_linked(
				Some("test".to_owned()),
				ClimateInterface {
					zone,
				},
				ClimateState {
					client: state.client.clone(),
					entity,
				},
				myself.get_cell(),
			)
			.await?;
		}

		Ok(())
	}

	async fn handle(
		&self,
		myself: ActorRef<Self::Msg>,
		message: Self::Msg,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		Ok(())
	}
}
