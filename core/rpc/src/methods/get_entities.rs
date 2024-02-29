use tokio::time::sleep;

use crate::{IntoFuture as Future, Output};

use super::{IntoMethod, Params};

pub struct GetEntities;

impl IntoMethod for GetEntities {
	fn into_method(_params: Params) -> Future<Output> {
		Box::pin(async move {
			sleep(std::time::Duration::from_secs(2)).await;
			Ok(serde_json::Value::Null)
		})
	}
}
