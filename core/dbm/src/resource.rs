use err::{Error, Result};

use futures::{Stream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::any::Any, method::Stream as DBStream, Notification};

use crate::{IntoFuture, DB};

pub trait Base: Sized + Serialize + DeserializeOwned + Send + Sync {
	const RESOURCE: &'static str;

	fn fetch(id: crate::Id) -> IntoFuture<'static, Result<Option<Self>, Error>> {
		let db = &DB;

		Box::pin(async move { Ok(db.select((Self::RESOURCE, id.to_raw())).await?) })
	}

	fn fetch_all<'r>() -> IntoFuture<'r, Result<Vec<Self>, Error>> {
		let db = &DB;

		Box::pin(async move { Ok(db.select(Self::RESOURCE).await?) })
	}

	fn stream() -> IntoFuture<'static, Result<impl Stream<Item = Notification<Self>>, Error>>
	where
		Self: Unpin,
	{
		let db = &DB;

		Box::pin(async move {
			let stream: DBStream<'static, Any, Vec<Self>> =
				db.select(Self::RESOURCE).live().await?;

			Ok(stream.filter_map(|n| async move { n.ok() }))
		})
	}
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
}
