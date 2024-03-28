use std::borrow::Cow;

use serde::de::DeserializeOwned;

use vnd::sandbox::SandboxError;

use super::{Client, Payload};

pub mod bridge_devices;
pub mod bridge_groups;

pub struct Method<'a, T>
where
	T: DeserializeOwned,
{
	pub client: Cow<'a, Client>,
	pub payload: T,
}

impl<'a, T> Method<'a, T>
where
	T: DeserializeOwned,
{
	pub fn new(client: &'a Client, payload: Payload) -> Result<Self, SandboxError> {
		Ok(Self {
			client: Cow::Borrowed(client),
			payload: serde_json::from_slice(&payload)?,
		})
	}
}
