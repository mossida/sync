use bus::Event;
use ractor::async_trait;

use sandbox::context::Context;
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, hash::Hash};
use svc::{r#type::ServiceType, Service};
use tracing::info;
use vendors::Vendors;

mod r#macro;

pub mod component;
pub mod sandbox;
pub mod spawner;
pub mod vendors;

pub enum VendorMessage {}

#[async_trait]
#[allow(unused_variables)]
pub trait Vendor: 'static + Send + Sync + Clone + Default {
	type Configuration: Serialize + DeserializeOwned + Hash + Clone + Send + Sync;
	type Message: From<VendorMessage> + From<Event> + Send + Sync;

	/* CONSTANTS */
	const NAME: &'static str;
	const VENDOR: Vendors;

	/* CONFIGURATION */
	const SUBSCRIBE_BUS: bool = false;
	const USES_IO: bool = false;

	const RETRIES: u8 = 3;
	const POLLING_INTERVAL: u64 = 0; // 0 means no polling, in seconds

	/* REGISTER FUNCTIONS */
	async fn services(&self) -> Vec<ServiceType> {
		vec![]
	}

	async fn triggers(&self) -> Vec<ServiceType> {
		vec![]
	}

	async fn setup(&self, config: Self::Configuration) {
		info!("{} setup", Self::NAME);
	}

	// Will be called every interval if
	// polling is enabled
	// Otherwise, it will be called once to completion
	// after the vendor is initialized
	async fn run(&self) {
		info!("{} run", Self::NAME);
	}

	async fn on_message(&self, ctx: &Context, msg: Self::Message) {}

	async fn on_service_call(&self, service: Service) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	async fn on_event(&self, event: Event) {}

	async fn stop(&self) {}
}
