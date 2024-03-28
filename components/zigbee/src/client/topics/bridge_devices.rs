use std::{
	future::{Future, IntoFuture},
	pin::Pin,
};

use vnd::sandbox::SandboxError;

use crate::client::payload::device::Device;

use super::Method;

pub type BridgeDevices<'a> = Method<'a, Vec<Device>>;

impl<'a> IntoFuture for BridgeDevices<'a> {
	type Output = Result<(), SandboxError>;
	type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync>>;

	fn into_future(self) -> Self::IntoFuture {
		todo!()
	}
}
