use dbm::{
	relation::Relation,
	resource::{Base, Resource},
	Id,
};
use serde::{Deserialize, Serialize};

use crate::{Class, Entity, State};

#[derive(Serialize, Deserialize)]
pub struct Device {
	id: dbm::Id,
	name: String,
	serial: String,
	model: String,
	manufacturer: String,
	sw_version: Option<String>,
	hw_version: Option<String>,
}

impl Default for Device {
	fn default() -> Self {
		Self {
			id: Id::rand(),
			name: Default::default(),
			serial: Default::default(),
			model: Default::default(),
			manufacturer: Default::default(),
			sw_version: Default::default(),
			hw_version: Default::default(),
		}
	}
}

impl Device {
	pub fn new(id: Option<Id>) -> Self {
		Self {
			id: id.unwrap_or(Id::rand()),
			..Default::default()
		}
	}
}

impl Base for Device {
	const RESOURCE: &'static str = "device";
}

impl Resource for Device {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

//
impl<C, S> Relation<Entity<C, S>> for Device
where
	C: Class,
	S: State,
{
	const RELATION: &'static str = "updates";
}
