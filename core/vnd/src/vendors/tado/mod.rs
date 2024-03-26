use ractor::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
	component::Component,
	sandbox::{actor::SandboxArguments, SandboxError},
	RefContext, Vendor,
};

use super::Vendors;

pub type Tado = Component<TadoVendor>;

#[derive(Clone, Hash, Deserialize, Serialize)]
pub struct TadoConfig {}

#[derive(Clone, Default)]
pub struct TadoVendor {}

#[async_trait]
impl Vendor for TadoVendor {
	type Configuration = TadoConfig;

	type Context = ();
	type PollData = ();

	const NAME: &'static str = "tado";
	const VENDOR: Vendors = Vendors::Tado;
	const SUBSCRIBE_BUS: bool = false;

	async fn initialize(&self, _: SandboxArguments<Self>) -> Result<Self::Context, SandboxError> {
		Ok(())
	}

	async fn poll(&self, _: RefContext<Self>) -> Result<Option<Self::PollData>, SandboxError> {
		Ok(None)
	}
}
