use bus::Event;
use err::{Error, Result};
use ractor::Actor;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{
	hash::{DefaultHasher, Hash, Hasher},
	marker::PhantomData,
};

mod r#macro;

pub mod vendors;

pub enum VendorMessage {}

pub trait Class: Actor<Msg = Self::Message, Arguments = ()> + Clone {
	type Configuration: Serialize + DeserializeOwned + Hash;
	type Message: From<VendorMessage> + From<Event>;

	const NAME: &'static str;

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
pub struct Vendor<C>
where
	C: Class,
{
	class: PhantomData<C>,
}

impl<C> Vendor<C>
where
	C: Class,
{
	/// Creates an instance of the provided class and spawns its actor.
	/// It requires a configuration as every instance is different from another
	/// based on its configuration.
	pub async fn build(config: Value) -> Result<Self, Error> {
		let class = C::new(serde_json::from_value(config)?);

		let _ = C::spawn(Some(class.name()), class.clone(), ()).await;

		let bus = bus::get();
		bus.emit(bus::Event::VendorStart {
			name: class.name(),
		});

		Ok(Self {
			class: PhantomData,
		})
	}
}
