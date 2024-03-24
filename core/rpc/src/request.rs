use std::{future::IntoFuture, pin::Pin};

use futures::Future;
use serde::Deserialize;

use crate::{methods::Method, Output};

#[derive(Deserialize)]
pub struct Request {
	pub id: u64,
	#[serde(flatten)]
	pub method: Method,
}

impl IntoFuture for Request {
	type Output = Output;
	type IntoFuture = Pin<Box<dyn Future<Output = Output> + Send + Sync>>;

	fn into_future(self) -> Self::IntoFuture {
		self.method.into_future()
	}
}
