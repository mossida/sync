mod get_entities;

use serde::Deserialize;
use serde_json::Value;
use std::future::IntoFuture;

use crate::{IntoFuture as Future, Output};

use self::get_entities::GetEntities;

fn resolve(method: &str, params: Option<Value>) -> Future<Output> {
	match method {
		"get_entities" => GetEntities::into_method(params),
		_ => {
			unreachable!()
		}
	}
}

pub trait IntoMethod {
	fn into_method(params: Option<Value>) -> Future<Output>;
}

#[derive(Debug, Deserialize)]
pub struct Method {
	pub method: String,
	pub params: Option<Value>,
}

impl IntoFuture for Method {
	type Output = Output;
	type IntoFuture = Future<Self::Output>;

	fn into_future(self) -> Self::IntoFuture {
		resolve(self.method.as_str(), self.params)
	}
}
