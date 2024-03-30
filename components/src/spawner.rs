use std::future::Future;

use dbm::{resource::Base, DB};
use enum_dispatch::enum_dispatch;
use err::Error;
use serde::{Deserialize, Serialize};
use vnd::{component::Component, Vendor};

#[enum_dispatch(Components)]
#[trait_variant::make(Send)]
pub trait Spawner {
	async fn spawn(&self) -> Result<(), Error>;
}

// Implement spawner for every component
impl<V> Spawner for Component<V>
where
	V: Vendor,
{
	fn spawn(&self) -> impl Future<Output = Result<(), Error>> + Send {
		self.build()
	}
}

pub async fn init() -> Result<(), Error> {
	let db = &DB;
	let components: Vec<Components> = db.select(Component::<()>::RESOURCE).await?;

	for component in components {
		component.spawn().await?;
	}

	Ok(())
}

#[enum_dispatch]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Components {
	Zigbee(Component<crate::zigbee::Zigbee>),
}
