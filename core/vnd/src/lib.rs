use bus::Event;
use serde::{de::DeserializeOwned, Serialize};
use std::hash::Hash;

use vendors::Vendors;

mod r#macro;

pub mod component;
pub mod sandbox;
pub mod spawner;
pub mod vendors;

pub enum VendorMessage {}

pub trait Vendor: 'static + Send + Sync + Clone + Default {
	type Configuration: Serialize + DeserializeOwned + Hash + Clone + Send + Sync;
	type Message: From<VendorMessage> + From<Event> + Send + Sync;

	const NAME: &'static str;
	const VENDOR: Vendors;
	const SUBSCRIBE_BUS: bool = false;
}
