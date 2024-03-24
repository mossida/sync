use bus::Event;
use component::Component;
use ractor::async_trait;

use sandbox::{actor::SandboxArguments, SandboxError};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashSet, hash::Hash, time::Duration};
use svc::{r#type::ServiceType, Service};
use tracing::warn;
use trg::Trigger;
use vendors::Vendors;

mod r#macro;

pub mod component;
pub mod sandbox;
pub mod spawner;
pub mod vendors;

pub static SCOPE: &str = "vendors";
pub static SANDBOX_GROUP: &str = "sandboxes";

#[async_trait]
#[allow(unused_variables)]
/// A trait representing a vendor.
pub trait Vendor: 'static + Send + Sync + Clone + Default {
	/// The configuration type associated with the vendor.
	type Configuration: Serialize + DeserializeOwned + Hash + Clone + Send + Sync;

	type Context: Send + Sync;
	type PollData: Send + Sync;

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
	async fn services(&self) -> HashSet<ServiceType> {
		Default::default()
	}

	/// Get the triggers for the vendor.
	async fn triggers(&self, instance: &Component<Self>) -> HashSet<Trigger> {
		Default::default()
	}

	/// Perform setup operations for the vendor.
	async fn initialize(
		&self,
		args: &SandboxArguments<Self>,
	) -> Result<Self::Context, SandboxError>;

	/// Main function where the vendor logic is executed.
	/// This function should not create any resources.
	/// It should only fetch data whether is pushing or polling
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
	async fn poll(&self, ctx: &mut Self::Context) -> Result<Self::PollData, SandboxError>;

	async fn process(
		&self,
		ctx: &mut Self::Context,
		data: Self::PollData,
	) -> Result<(), SandboxError> {
		warn!("The vendor {} is not handling the data", Self::NAME);

		Ok(())
	}

	/// Handle a service call.
	async fn on_service_call(&self, service: Service) -> Result<(), SandboxError> {
		warn!("A service got called, but the vendor {} is not handling services", Self::NAME);

		Ok(())
	}

	/// Handle an event.
	async fn on_event(&self, event: Event) {}

	/// Stop the vendor.
	async fn stop(&self) {}
}
