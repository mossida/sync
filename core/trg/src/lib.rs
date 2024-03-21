use bus::Event;
use dbm::resource::{Base, Resource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum TriggerOrigin {
	System,
	Component(dbm::Id),
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Trigger {
	id: dbm::Id,
	name: String,
	r#type: Event,
	origin: TriggerOrigin,
}

impl Base for Trigger {
	const RESOURCE: &'static str = "trigger";
}

impl Resource for Trigger {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

impl Trigger {
	pub fn check(&self, event: Event) -> bool {
		event == self.r#type
	}
}
