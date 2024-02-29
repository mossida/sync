use cls::class::any::Any;
use dbm::resource::Base;

use crate::{IntoFuture as Future, Output};

use super::{IntoMethod, Params};

pub struct GetEntities;

async fn run() -> Output {
	let result = Any::fetch_all().await?;
	let data = serde_json::to_value(result)?;

	Ok(data)
}

impl IntoMethod for GetEntities {
	fn into_method(_params: Params) -> Future<Output> {
		Box::pin(run())
	}
}
