use err::{Error, Result};
use surrealdb::Response;

use crate::{resource::Resource, DB};

#[trait_variant::make(Send + Sync)]
pub trait Relation<W: Resource>: Resource {
	/// indicates relation name
	const RELATION: &'static str;
	/// indicates if the relation is inverted
	const INVERTED: bool = false;

	async fn relate(&self, with: &W) -> Result<Response, Error> {
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

		async move { Ok(future.await?) }
	}

	async fn relationships(&self) -> Result<Vec<W>, Error> {
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

		async move {
			let mut response = future.await?;
			Ok(response.take(0)?)
		}
	}
}
