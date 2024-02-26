use std::marker::PhantomData;

use cls::device::Device;
use dbm::{
	relation::Relation,
	resource::{Base, Resource},
};
use err::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
	vendors::{implement, Vendors},
	Vendor,
};

/// Represents an instance of a class.
#[derive(Serialize, Deserialize, Debug)]
pub struct Component<V> {
	id: dbm::Id,
	r#type: Vendors,
	config: Value,
	#[serde(skip)]
	vendor: PhantomData<V>,
}

impl<V> Component<V> {
	pub async fn implement(&self) -> err::Result<(), err::Error> {
		implement(&self.r#type, self.config.clone()).await
	}
}

impl<V> Component<V>
where
	V: Vendor,
{
	pub fn new(config: Value) -> Result<Self, Error> {
		Ok(Self {
			id: dbm::Id::rand(),
			r#type: V::VENDOR,
			config,
			vendor: PhantomData,
		})
	}

	/// Creates an instance of the provided class and spawns its actor.
	/// It requires a configuration as every instance is different from another
	/// based on its configuration.
	pub async fn build(&self) -> Result<(), Error> {
		let class = V::new(serde_json::from_value(self.config.clone())?);

		let _ = V::spawn(Some(class.name()), class.clone(), ()).await;

		let bus = bus::get();
		bus.emit(bus::Event::VendorStart {
			name: class.name(),
		});

		Ok(())
	}
}

impl<V> Base for Component<V>
where
	V: Send + Sync,
{
	const RESOURCE: &'static str = "component";
}

impl<V> Resource for Component<V>
where
	V: Send + Sync,
{
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

impl<V> Relation<Device> for Component<V>
where
	V: Send + Sync,
{
	const RELATION: &'static str = "controls";
}
