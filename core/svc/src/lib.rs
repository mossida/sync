use dbm::resource::{Base, Resource};
use ractor::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
	id: dbm::Id,
	component: dbm::Id,
	service_type: dbm::Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceType {
	pub id: dbm::Id,
	pub name: String,
}

impl Service {
	pub fn execute<T>(&self)
	where
		T: From<Service> + Message,
	{
	}

	pub fn run(&self) {
		dbg!("Running service");
	}
}

impl Base for Service {
	const RESOURCE: &'static str = "service";
}

impl Resource for Service {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}
