mod get;

use dashmap::DashMap;
use futures::Future;
use serde::Deserialize;
use serde_json::Value;
use std::{future::IntoFuture, pin::Pin};

use crate::{Output, RpcError};

use self::get::*;

async fn resolve(method: Method) -> Output {
	let Method {
		method,
		params,
	} = method;

	match method.as_str() {
		"get_resource" => GetResource::into_method(params).await,
		_ => Err(RpcError::MethodNotFound),
	}
}

pub trait IntoMethod {
	fn into_method(params: Params) -> impl Future<Output = Output>;
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
	type IntoFuture = Pin<Box<dyn Future<Output = Output> + Send + Sync>>;

	fn into_future(self) -> Self::IntoFuture {
		Box::pin(resolve(self))
	}
}
