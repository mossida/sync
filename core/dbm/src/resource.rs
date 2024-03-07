use err::{Error, Result};

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::any::Any, method::Stream};

use crate::{IntoFuture, DB};

pub trait Base: Sized + Serialize + DeserializeOwned + Send + Sync {
	const RESOURCE: &'static str;

	fn fetch_all<'r>() -> IntoFuture<'r, Result<Vec<Self>, Error>> {
		let db = &DB;

		Box::pin(async move { Ok(db.select(Self::RESOURCE).await?) })
	}

	// TODO: implement method that subscribe changes to main BUS
}

pub trait Resource: Base {
	fn id(&self) -> &crate::Id;

	fn exists(&self) -> IntoFuture<'_, Result<bool, Error>> {
		let db = &DB;
		let id = self.id().to_owned();

		let future = db
			.query("SELECT count(*) FROM $resource WHERE id = $id")
			.bind(("resource", Self::RESOURCE))
			.bind(("id", id.to_raw()));

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

		Box::pin(async move { Ok(db.delete((Self::RESOURCE, id.to_raw())).await?) })
	}

	fn update(&self) -> IntoFuture<'_, Result<Option<Self>, Error>> {
		let db = &DB;
		let id = self.id().to_owned();

		Box::pin(async move { Ok(db.update((Self::RESOURCE, id.to_raw())).merge(self).await?) })
	}

	fn stream() -> IntoFuture<'static, Result<Stream<'static, Any, Vec<Self>>, Error>> {
		let db = &DB;

		Box::pin(async move { Ok(db.select(Self::RESOURCE).live().await?) })
	}
}
