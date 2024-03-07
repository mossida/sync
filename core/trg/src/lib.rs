use dbm::resource::{Base, Resource};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum TriggerType {
	Event,

	#[default]
	Manual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TriggerOrigin {
	System,
	Component(dbm::Id),
}

#[derive(Debug, Serialize, Deserialize)]
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
