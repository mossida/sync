use err::{Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::sql::Id;

use crate::{IntoFuture, DB};

pub trait Base: Sized + Serialize + DeserializeOwned + Send + Sync {
	const RESOURCE: &'static str;

	// TODO: implement method that subscribe changes to main BUS
}

pub trait Resource: Base {
	fn id(&self) -> &Id;

	fn exists(&self) -> IntoFuture<'_, Result<bool, Error>> {
		let db = &DB;
		let id = self.id().to_owned();

		let future = db
			.query("SELECT count(*) FROM $resource WHERE id = $id")
			.bind(("resource", Self::RESOURCE))
			.bind(("id", id));

		Box::pin(async move {
			let mut response = future.await?;
			let count: Option<i8> = response.take(0)?;

			if let Some(count) = count {
				Ok(count == 1)
			} else {
				Ok(false)
			}
		})
	}

	fn create(&self) -> IntoFuture<'_, Result<Vec<Self>, Error>> {
		let db = &DB;

		Box::pin(async move { Ok(db.create(Self::RESOURCE).content(&self).await?) })
	}

	fn delete(&self) -> IntoFuture<'_, Result<Option<Self>, Error>> {
		let db = &DB;
		let id = self.id().to_owned();

		Box::pin(async move { Ok(db.delete((Self::RESOURCE, id)).await?) })
	}

	fn update(&self) -> IntoFuture<'_, Result<Option<Self>, Error>> {
		let db = &DB;
		let id = self.id().to_owned();

		Box::pin(async move { Ok(db.update((Self::RESOURCE, id)).merge(self).await?) })
	}
}
