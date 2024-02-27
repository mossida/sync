use dbm::resource::Base;
use err::Error;

use crate::{component::Component, vendors::any::Any as GenericComponent};

pub async fn init() -> Result<(), Error> {
	let vendors: Vec<GenericComponent> = Component::fetch_all().await?;

	for v in vendors.iter() {
		v.implement().await?;
	}

	Ok(())
}
