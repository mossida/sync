use serde_json::Value;

use crate::{IntoFuture as Future, Output};

use super::IntoMethod;

pub struct GetEntities;

impl IntoMethod for GetEntities {
	fn into_method(_params: Option<Value>) -> Future<Output> {
		unimplemented!()
	}
}
