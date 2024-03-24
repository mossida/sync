use dbm::resource::{Base, Resource};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct Triggers {
	id: dbm::Id,
	at: Datetime,
	trigger: Thing,
	component: Thing,
}

impl Base for Triggers {
	const RESOURCE: &'static str = "triggers";
}

impl Resource for Triggers {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

/*impl Triggers {
	pub fn new(trigger: Thing, component: Thing) -> Self {
		Self {
			id: Default::default(),
			at: Default::default(),
			trigger,
			component,
		}
	}
}*/
