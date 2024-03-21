use bus::Event;
use ractor::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;
use svc::r#type::ServiceType;
use vendors::Vendors;

mod r#macro;

pub mod component;
pub mod sandbox;
pub mod spawner;
pub mod vendors;

pub enum VendorMessage {}

#[async_trait]
pub trait Vendor: 'static + Send + Sync + Clone + Default {
	type Configuration: Serialize + DeserializeOwned + Hash + Clone + Send + Sync;
	type Message: From<VendorMessage> + From<Event> + Send + Sync;

	/* CONSTANTS */
	const NAME: &'static str;
	const VENDOR: Vendors;

	/* CONFIGURATION */
	const SUBSCRIBE_BUS: bool = false;
	const POLLING_INTERVAL: usize = 0; // 0 means no polling, in seconds

	/* REGISTER FUNCTIONS */
	async fn services(&self) -> Vec<ServiceType> {
		vec![]
	}

	async fn triggers(&self) -> Vec<ServiceType> {
		vec![]
	}

	async fn setup(&self) {}

	#[allow(unused_variables)]
	async fn on_message(&self, msg: Self::Message) {}

	#[allow(unused_variables)]
	async fn on_event(&self, event: Event) {}

	async fn stop(&self) {}
}
