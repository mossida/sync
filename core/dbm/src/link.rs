use err::Error;

use crate::{resource::Resource, Id, IntoFuture};

pub trait Link<W: Resource>: Resource {
	fn id(&self) -> Id;

	fn fetch(&self) -> IntoFuture<'_, Result<Option<W>, Error>> {
		W::fetch(Link::id(self))
	}
}
