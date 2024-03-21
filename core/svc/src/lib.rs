use dbm::{
	link::Link,
	resource::{Base, Resource},
};
use r#type::ServiceType;
use ractor::Message;
use serde::{Deserialize, Serialize};

mod r#type;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
	id: dbm::Id,
	component: dbm::Id,
	service_type: dbm::Id,
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

impl Link<ServiceType> for Service {
	fn id(&self) -> dbm::Id {
		self.service_type.to_owned()
	}
}
