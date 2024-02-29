mod get_entities;

use dashmap::DashMap;
use serde::Deserialize;
use serde_json::Value;
use std::future::IntoFuture;

use crate::{IntoFuture as Future, Output, RpcError};

use self::get_entities::GetEntities;

fn resolve(method: &str, params: Params) -> Future<Output> {
	match method {
		"get_entities" => GetEntities::into_method(params),
		_ => Box::pin(async move { Err(RpcError::MethodNotFound) }),
	}
}

pub trait IntoMethod {
	fn into_method(params: Params) -> Future<Output>;
}

pub type Params = DashMap<String, Value>;

#[derive(Debug, Deserialize)]
pub struct Method {
	pub method: String,

	#[serde(default)]
	pub params: Params,
}

impl IntoFuture for Method {
	type Output = Output;
	type IntoFuture = Future<Self::Output>;

	fn into_future(self) -> Self::IntoFuture {
		resolve(self.method.as_str(), self.params)
	}
}
