use std::{
	future::{Future, IntoFuture},
	pin::Pin,
};

use crate::sandbox::SandboxError;

use super::Method;

pub type BridgeGroups<'a> = Method<'a, String>;

impl<'a> IntoFuture for BridgeGroups<'a> {
	type Output = Result<(), SandboxError>;
	type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + Sync>>;

	fn into_future(self) -> Self::IntoFuture {
		todo!()
	}
}
