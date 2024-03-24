use err::{Error, Result};
use surrealdb::Response;

use crate::{resource::Resource, IntoFuture, DB};

pub trait Relation<W: Resource>: Resource {
	/// indicates relation name
	const RELATION: &'static str;
	/// indicates if the relation is inverted
	const INVERTED: bool = false;

	fn relate(&self, with: &W) -> IntoFuture<'_, Result<Response, Error>> {
		let db = &DB;
		let query = if Self::INVERTED {
			"RELATE $with->$relation->$me"
		} else {
			"RELATE $me->$relation->$with"
		};

		let future = db
			.query(query)
			.bind(("me", (Self::RESOURCE, self.id())))
			.bind(("relation", Self::RELATION))
			.bind(("with", (W::RESOURCE, with.id())));

		Box::pin(async move { Ok(future.await?) })
	}

	fn relationships(&self) -> IntoFuture<'_, Result<Vec<W>, Error>> {
		let db = &DB;
		let query = if Self::INVERTED {
			"SELECT ->$relation->$me FROM $with"
		} else {
			"SELECT ->$relation->$with FROM $me"
		};

		let future = db
			.query(query)
			.bind(("me", (Self::RESOURCE, self.id())))
			.bind(("relation", Self::RELATION))
			.bind(("with", W::RESOURCE));

		Box::pin(async move {
			let mut response = future.await?;
			Ok(response.take(0)?)
		})
	}
}
