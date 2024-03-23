use bus::Event;
use ractor::async_trait;

use sandbox::{context::Context, SandboxError};
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, hash::Hash, time::Duration};
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
/// A trait representing a vendor.
pub trait Vendor: 'static + Send + Sync + Clone + Default {
	/// The configuration type associated with the vendor.
	type Configuration: Serialize + DeserializeOwned + Hash + Clone + Send + Sync;
	/// The message type associated with the vendor.
	type Message: From<VendorMessage> + From<Event> + Send + Sync;

	/* CONSTANTS */

	/// The name of the vendor.
	const NAME: &'static str;
	/// The vendor type.
	const VENDOR: Vendors;

	/* CONFIGURATION */

	/// The number of retries after poll function
	/// fails to execute.
	const RETRIES: u8 = 3;
	/// Whether to subscribe to the bus.
	const SUBSCRIBE_BUS: bool = false;
	/// The polling interval for the poll function.
	/// If set to 0, the run function will be called
	/// once to completion after the vendor is initialized
	const POLLING_INTERVAL: Duration = Duration::from_secs(0);

	/* REGISTER FUNCTIONS */

	/// Get the services provided by the vendor.
	async fn services(&self) -> Vec<ServiceType> {
		vec![]
	}

	/// Get the triggers for the vendor.
	async fn triggers(&self) -> Vec<ServiceType> {
		vec![]
	}

	/// Perform setup operations for the vendor.
	async fn initialize(&self, config: Self::Configuration) {
		info!("{} setup", Self::NAME);
	}

	/// Main function where the vendor logic is executed.
	/// This function should not create any resources.
	/// It should only fetch data and update states
	///
	/// If polling interval is set to 0, this function
	/// will be called once to completion after the vendor
	/// is initialized.
	///
	/// If polling interval is set to a non-zero value,
	/// this function will be called repeatedly at the
	/// specified interval. If the function takes more
	/// time to execute than the polling interval, the
	/// function will be called again after the previous
	/// execution completes.
	///
	/// If the function fails to execute, it will be
	/// retried RETRIES number of times.
	async fn poll(&self) -> Result<(), SandboxError> {
		info!("{} run", Self::NAME);
		Ok(())
	}

	/// Handle an incoming message.
	async fn on_message(&self, ctx: &Context, msg: Self::Message) {}

	/// Handle a service call.
	async fn on_service_call(&self, service: Service) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	/// Handle an event.
	async fn on_event(&self, event: Event) {}

	/// Stop the vendor.
	async fn stop(&self) {}
}
