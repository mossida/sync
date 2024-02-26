use bus::Event;
use ractor::Actor;
use serde::{de::DeserializeOwned, Serialize};
use std::hash::{DefaultHasher, Hash, Hasher};
use vendors::Vendors;

mod r#macro;

pub mod component;
pub mod spawner;
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
