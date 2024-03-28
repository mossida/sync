use std::marker::PhantomData;

use cls::device::Device;
use dbm::{
	relation::Relation,
	resource::{Base, Resource},
};
use err::Error;
use ractor::Actor;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, info};

use crate::{
	sandbox::{actor::SandboxArguments, Sandbox},
	Vendor,
};

/// Represents an instance of a class.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Component<V>
where
	V: Vendor,
{
	id: dbm::Id,
	config: Value,

	#[serde(skip)]
	marker: PhantomData<V>,
}

impl<V> Component<V>
where
	V: Vendor,
{
	pub fn new(config: Value) -> Result<Self, Error> {
		Ok(Self {
			id: dbm::Id::rand(),
			config,
			marker: PhantomData,
		})
	}

	/// Creates an instance of the provided class and spawns its actor.
	/// It requires a configuration as every instance is different from another
	/// based on its configuration.
	pub async fn build(self) -> Result<(), Error> {
		let name = self.id.to_raw();
		let configuration: V::Configuration = serde_json::from_value(self.config.clone())?;
		let vendor: V = Default::default();
		let sandbox = Sandbox::new(vendor);
		let spawn = Actor::spawn(
			Some(name.clone()),
			sandbox,
			SandboxArguments {
				component: self,
				configuration,
			},
		)
		.await;

		match spawn {
			Err(e) => error!("Couldn't spawn for {} because {}", name, e),
			Ok(_) => info!("Spawned actor for {}", name),
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
