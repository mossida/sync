use bus::Bus;
use dbm::{
	relation::Relation,
	resource::{Base, Resource},
};
use serde::{Deserialize, Serialize};
use svc::Service;
use trg::Trigger;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Automation {
	id: dbm::Id,
	name: String,
}

impl Base for Automation {
	const RESOURCE: &'static str = "automation";
}

impl Resource for Automation {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

impl Relation<Trigger> for Automation {
	const RELATION: &'static str = "upon";
}

impl Relation<Service> for Automation {
	const RELATION: &'static str = "executes";
}

pub async fn init() {
	let _ = bus::get();
	let _: Bus<Automation> = Bus::new();
}
