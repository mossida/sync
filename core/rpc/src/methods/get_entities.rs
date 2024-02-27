use crate::{IntoFuture as Future, Output};

use super::{IntoMethod, Params};

pub struct GetEntities;

impl IntoMethod for GetEntities {
	fn into_method(_params: Params) -> Future<Output> {
		unimplemented!()
	}
}
