use std::{any::TypeId, marker::PhantomData};

use cls::device::Device;
use dbm::{
	relation::Relation,
	resource::{Base, Resource},
};
use err::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, info};

use crate::{
	vendors::{any::AnyVendor, implement, Vendors},
	Vendor,
};

/// Represents an instance of a class.
#[derive(Serialize, Deserialize, Debug)]
pub struct Component<V>
where
	V: Vendor,
{
	id: dbm::Id,
	r#type: Vendors,
	config: Value,

	#[serde(skip)]
	marker: PhantomData<V>,
}

impl<V> Component<V>
where
	V: Vendor,
{
	pub async fn implement(&self) -> err::Result<(), err::Error> {
		// This should take ownership of the component and spawn the actor
		implement(&self.r#type, self.config.clone()).await
	}
}

impl<V> Component<V>
where
	V: Vendor,
{
	pub fn new(config: Value) -> Result<Self, Error> {
		if TypeId::of::<V>() == TypeId::of::<AnyVendor>() {
			unreachable!();
		}

		Ok(Self {
			id: dbm::Id::rand(),
			r#type: V::VENDOR,
			config,
			marker: PhantomData,
		})
	}

	/// Creates an instance of the provided class and spawns its actor.
	/// It requires a configuration as every instance is different from another
	/// based on its configuration.
	pub async fn build(&self) -> Result<(), Error> {
		if TypeId::of::<V>() == TypeId::of::<AnyVendor>() {
			unreachable!();
		}

		let class = V::new(serde_json::from_value(self.config.clone())?);
		let name = class.name();
		let spawn = V::spawn(Some(name.clone()), class.clone(), ()).await;
		if let Err(e) = spawn {
			error!("Couldn't spawn for {} because {}", name, e);
		} else {
			info!("Spawned actor for {}", name);
		}

		let bus = bus::get();
		bus.emit(bus::Event::VendorStart(name));

		Ok(())
	}
}

impl<V> Base for Component<V>
where
	V: Vendor,
{
	const RESOURCE: &'static str = "component";
}

impl<V> Resource for Component<V>
where
	V: Vendor,
{
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

impl<V> Relation<Device> for Component<V>
where
	V: Vendor,
{
	const RELATION: &'static str = "controls";
}
