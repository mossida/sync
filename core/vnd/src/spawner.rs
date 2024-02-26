use dbm::resource::Base;
use err::Error;
use futures::future::join_all;

use crate::component::Component;

pub async fn init() -> Result<(), Error> {
	// Use a generic type since we don't know the vendors
	// I don't like this solution, so we need to find a better one
	let components: Vec<Component<()>> = Component::fetch_all().await?;
	join_all(components.iter().map(|c| c.implement())).await;

	Ok(())
}
