use err::Error;

use crate::{resource::Resource, Id};

#[trait_variant::make(Send + Sync)]
pub trait Link<W: Resource>: Resource {
	fn id(&self) -> &Id;

	async fn fetch(&self) -> Result<Option<W>, Error> {
		W::fetch(Link::id(self))
	}
}
