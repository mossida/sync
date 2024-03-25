use ractor::async_trait;

use crate::{
	component::Component,
	sandbox::{actor::SandboxArguments, SandboxError},
	RefContext, Vendor,
};

use super::Vendors;

pub type Any = Component<AnyVendor>;

#[derive(Debug, Clone, Default)]
pub struct AnyVendor {}

#[async_trait]
impl Vendor for AnyVendor {
	type Configuration = ();

	type Context = ();
	type PollData = ();

	const NAME: &'static str = "any";
	const VENDOR: Vendors = Vendors::Any;

	async fn initialize(&self, _: &SandboxArguments<Self>) -> Result<Self::Context, SandboxError> {
		Ok(())
	}

	async fn poll(&self, _: RefContext<Self>) -> Result<Option<Self::PollData>, SandboxError> {
		Ok(None)
	}
}
