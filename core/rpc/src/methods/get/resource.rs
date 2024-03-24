use cls::class::any::Any;
use dbm::{resource::Base, DB};
use futures::Future;
use serde::{Deserialize, Serialize};
use trg::Trigger;

use crate::{
	methods::{IntoMethod, Params},
	Output, RpcError,
};

static PARAM_RESOURCE: &str = "resource";

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Resources {
	Entity(Any),
	Trigger(Trigger),
}

impl Resources {
	pub fn is_valid(resource: &str) -> bool {
		match resource {
			Any::RESOURCE => true,
			Trigger::RESOURCE => true,
			_ => false,
		}
	}
}

/// When the method is called, it will
/// fetch all resources of the given type.
/// from the database.
pub struct GetResource;

async fn run(params: Params) -> Output {
	let param =
		params.get(PARAM_RESOURCE).ok_or(RpcError::MissingParameter(PARAM_RESOURCE.to_string()))?;

	let resource = param
		.as_str()
		.filter(|r| Resources::is_valid(r))
		.ok_or(RpcError::InvalidParameter(PARAM_RESOURCE.to_string()))?;

	let db = &DB;
	let data: Vec<Resources> =
		db.select(resource).await.map_err(|e| RpcError::ProcessingError(e.into()))?;

	Ok(serde_json::to_value(data)?)
}

impl IntoMethod for GetResource {
	fn into_method(params: Params) -> impl Future<Output = Output> {
		run(params)
	}
}
