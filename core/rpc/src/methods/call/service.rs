use std::future::Future;

use svc::Service;
use vnd::sandbox::{Request, SandboxMessage};

use crate::{
	methods::{get_parameter, IntoMethod, Params},
	Output,
};

pub struct CallService;

async fn run(params: Params) -> Output {
	let svc: Service = get_parameter(&params, "service")?;
	svc.call(|port, service| SandboxMessage::Request(Request::Call(service), port)).await?;

	Ok(serde_json::Value::Null)
}

impl IntoMethod for CallService {
	fn into_method(params: Params) -> impl Future<Output = Output> {
		run(params)
	}
}
