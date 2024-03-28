use std::future::IntoFuture;

use err::{Error, Result};

use futures::{Stream, StreamExt, TryFutureExt};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::any::Any, method::Stream as DBStream, Notification};

use crate::DB;

#[trait_variant::make(Send + Sync)]
pub trait Base: Sized + Serialize + DeserializeOwned + Send + Sync {
	const RESOURCE: &'static str;

	async fn fetch(id: crate::Id) -> Result<Option<Self>, Error> {
		let db = &DB;
		db.select((Self::RESOURCE, id.to_raw())).into_future().map_err(Into::into)
	}

	async fn fetch_all<'r>() -> Result<Vec<Self>, Error> {
		let db = &DB;
		db.select(Self::RESOURCE).into_future().map_err(Into::into)
	}

	async fn stream() -> Result<impl Stream<Item = Notification<Self>>, Error>
	where
		Self: Unpin,
	{
		let db = &DB;

		async move {
			let stream: DBStream<'static, Any, Vec<Self>> =
				db.select(Self::RESOURCE).live().await?;

			Ok(stream.filter_map(|n| async move { n.ok() }))
		}
	}
}

#[trait_variant::make(Send + Sync)]
pub trait Resource: Base {
	fn id(&self) -> &crate::Id;

	async fn exists(&self) -> Result<bool, Error> {
		let db = &DB;
		let id = self.id().to_owned();

		let future = db
			.query("SELECT count(*) FROM $resource WHERE id = $id")
			.bind(("resource", Self::RESOURCE))
			.bind(("id", id.to_raw()));

		async move {
			let mut response = future.await?;
			let count: Option<i8> = response.take(0)?;

			if let Some(count) = count {
				Ok(count == 1)
			} else {
				Ok(false)
			}
		}
	}

	async fn create(&self) -> Result<Vec<Self>, Error> {
		let db = &DB;
		db.create(Self::RESOURCE).content(&self).into_future().map_err(Into::into)
	}

	async fn delete(&self) -> Result<Option<Self>, Error> {
		let db = &DB;
		let id = self.id().to_owned();
		db.delete((Self::RESOURCE, id.to_raw())).into_future().map_err(Into::into)
	}

	async fn update(&self) -> Result<Option<Self>, Error> {
		let db = &DB;
		let id = self.id().to_owned();
		db.update((Self::RESOURCE, id.to_raw())).merge(self).into_future().map_err(Into::into)
	}
}
