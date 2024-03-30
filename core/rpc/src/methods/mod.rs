mod call;
mod get;

use dashmap::DashMap;
use futures::Future;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use std::{future::IntoFuture, pin::Pin};

use crate::{Output, RpcError};

use self::call::*;
use self::get::*;

fn get_parameter<T>(params: &Params, key: &'static str) -> Result<T, RpcError>
where
	T: DeserializeOwned,
{
	let param = params.get(key).ok_or(RpcError::MissingParameter(key))?;
	let value: T = serde_json::from_value(param.value().clone())
		.map_err(|_| RpcError::InvalidParameter(key))?;

	Ok(value)
}

async fn resolve(method: Method) -> Output {
	let Method {
		method,
		params,
	} = method;

	match method.as_str() {
		"get_resource" => GetResource::into_method(params).await,
		"call_service" => CallService::into_method(params).await,
		_ => Err(RpcError::MethodNotFound),
	}
}

pub trait IntoMethod {
	async fn into_method(params: Params) -> Output;
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
