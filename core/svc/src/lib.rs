use dbm::{
	link::Link,
	resource::{Base, Resource},
};
use r#type::ServiceType;
use serde::{Deserialize, Serialize};

pub mod r#type;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
	id: dbm::Id,
	pub component: dbm::Id,
	pub service_type: dbm::Id,
}

impl Service {
	pub fn is(&self, service_type: &ServiceType) -> bool {
		self.service_type == service_type.id
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
