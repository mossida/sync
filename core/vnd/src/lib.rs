use bus::Event;
use cls::device::Device;
use dbm::{
	relation::Relation,
	resource::{Base, Resource},
};
use err::{Error, Result};
use ractor::Actor;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::{
	hash::{DefaultHasher, Hash, Hasher},
	marker::PhantomData,
};
use vendors::Vendors;

mod r#macro;

pub mod vendors;

pub enum VendorMessage {}

pub trait Vendor: Actor<Msg = Self::Message, Arguments = ()> + Clone {
	type Configuration: Serialize + DeserializeOwned + Hash + Clone + Send + Sync;
	type Message: From<VendorMessage> + From<Event>;

	const NAME: &'static str;
	const VENDOR: Vendors;

	fn new(config: Self::Configuration) -> Self;

	fn configuration(&self) -> Self::Configuration;

	/// Generates the name of the class based on its static name
	/// and the hash of its configuration.
	/// So that every unique configuration will spawn into a different actor.
	fn name(&self) -> String {
		let mut hasher = DefaultHasher::new();
		let config = self.configuration();
		config.hash(&mut hasher);

		format!("{}-{}", Self::NAME, hasher.finish())
	}
}

/// Represents an instance of a class.
#[derive(Serialize, Deserialize, Debug)]
pub struct Component<V> {
	id: dbm::Id,
	r#type: Vendors,
	config: Value,
	#[serde(skip)]
	vendor: PhantomData<V>,
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
