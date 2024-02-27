use std::future::IntoFuture;

use serde::Deserialize;

use crate::{methods::Method, IntoFuture as Future, Output};

#[derive(Deserialize)]
pub struct Request {
	pub id: u64,
	#[serde(flatten)]
	pub method: Method,
}

impl IntoFuture for Request {
	type Output = Output;
	type IntoFuture = Future<Self::Output>;

	fn into_future(self) -> Self::IntoFuture {
		self.method.into_future()
	}
}
