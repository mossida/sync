use bus::Event;
use dbm::{
	relation::Relation,
	resource::{Base, Resource},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Hash, Clone)]
pub enum TriggerType {
	Event(Event),

	#[default]
	Manual,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum TriggerOrigin {
	System,
	Component(dbm::Id),
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Trigger {
	id: dbm::Id,
	name: String,
	r#type: TriggerType,
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
