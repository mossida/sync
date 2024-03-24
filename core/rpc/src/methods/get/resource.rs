use atm::Automation;
use cls::{class::any::Any, device::Device};
use dbm::{resource::Base, DB};
use futures::Future;
use serde::{Deserialize, Serialize};
use svc::{r#type::ServiceType, Service};
use trg::Trigger;

use crate::{
	methods::{get_parameter, IntoMethod, Params},
	Output, RpcError,
};

static RESOURCE_PARAM: &str = "resource";

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Resources {
	Entity(Any),
	Device(Device),
	Trigger(Trigger),
	Automation(Automation),
	Service(Service),
	ServiceType(ServiceType),
}

impl Resources {
	pub fn is_valid(resource: &str) -> bool {
		matches!(
			resource,
			Any::RESOURCE
				| Device::RESOURCE
				| Trigger::RESOURCE
				| Automation::RESOURCE
				| Service::RESOURCE
				| ServiceType::RESOURCE
		)
	}
}

/// When the method is called, it will
/// fetch all resources of the given type.
/// from the database.
pub struct GetResource;

async fn run(params: Params) -> Output {
	let resource = get_parameter(&params, RESOURCE_PARAM).and_then(|param: String| {
		Resources::is_valid(param.as_str())
			.then_some(param)
			.ok_or(RpcError::InvalidParameter(RESOURCE_PARAM))
	})?;

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
