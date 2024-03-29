use bus::Event;

use ractor::async_trait;

use sandbox::{actor::SandboxArguments, SandboxError};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashSet, hash::Hash, sync::Arc, time::Duration};
use svc::{r#type::ServiceType, Service};
use tracing::warn;
use trg::Trigger;

pub mod component;
pub mod sandbox;

pub static SCOPE: &str = "vendors";
pub static SANDBOX_GROUP: &str = "sandboxes";

pub type RefContext<V> = Arc<<V as Vendor>::Context>;

#[async_trait]
#[allow(unused_variables)]
/// A trait representing a vendor.
pub trait Vendor: 'static + Send + Sync + Clone + Default {
	/// The configuration type associated with the vendor.
	type Configuration: Serialize + DeserializeOwned + Hash + Clone + Send + Sync;

	type Context: Send + Sync;
	type PollData: Send + Sync;
	/* CONFIGURATION */
	const NAME: &'static str;

	/// Whether to subscribe to the bus.
	const SUBSCRIBE_BUS: bool = false;
	/// The polling interval for the poll function.
	/// If set to 0, the run function will be called
	/// once to completion after the vendor is initialized
	const POLLING_INTERVAL: Duration = Duration::from_millis(1);

	const STOP_ON_ERROR: bool = false;

	/* REGISTER FUNCTIONS */

	/// Get the services provided by the vendor.
	async fn services() -> HashSet<ServiceType> {
		Default::default()
	}

	/// Get the triggers for the vendor.
	async fn triggers() -> HashSet<Trigger> {
		Default::default()
	}

	/// Perform setup operations for the vendor.
	async fn initialize(args: SandboxArguments<Self>) -> Result<Self::Context, SandboxError>;

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
	async fn poll(ctx: RefContext<Self>) -> Result<Option<Self::PollData>, SandboxError>;

	async fn consume(ctx: RefContext<Self>, data: Self::PollData) -> Result<(), SandboxError> {
		warn!("The vendor is not handling the data");

		Ok(())
	}

	/// Handle a service call.
	async fn on_service_call(service: Service) -> Result<(), SandboxError> {
		warn!("A service got called, but the vendor is not handling services");

		Ok(())
	}

	/// Handle an event.
	async fn on_event(event: Event) {}

	/// Stop the vendor.
	async fn stop() {}
}
