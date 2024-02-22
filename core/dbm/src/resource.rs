use err::{Error, Result};
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::pin::Pin;
use surrealdb::sql::Id;

use crate::DB;

/// Internal type to indicate an async method
type IntoFuture<'r, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'r>>;

pub trait Base: Sized + Serialize + DeserializeOwned + Send + Sync {
	const RESOURCE: &'static str;

	// TODO: implement method that subscribe changes to main BUS
}

pub trait Resource: Base {
	fn id(&self) -> &Id;

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
