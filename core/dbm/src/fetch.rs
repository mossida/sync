use std::borrow::Cow;

use either::Either;
use err::Error;
use serde::{Deserialize, Serialize};

use crate::{resource::Resource, Id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fetch<S: Resource>(
	#[serde(bound = "")]
	#[serde(with = "either::serde_untagged")]
	Either<S, Id>,
);

impl<S> Fetch<S>
where
	S: Resource + ToOwned<Owned = S>,
{
	pub fn id(&self) -> &Id {
		match &self.0 {
			Either::Left(s) => s.id(),
			Either::Right(id) => id,
		}
	}

	pub async fn fetch(&self) -> Result<Cow<'_, S>, Error> {
		match &self.0 {
			Either::Left(s) => Ok(Cow::Borrowed(s)),
			Either::Right(id) => {
				let s = S::fetch(id)
					.await
					.ok()
					.flatten()
					.ok_or(Error::CustomError("Invalid fetch".to_string()))?;

				Ok(Cow::Owned(s))
			}
		}
	}
}
