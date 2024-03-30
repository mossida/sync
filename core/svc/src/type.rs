use dbm::resource::{Base, Resource};

use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceType {
	pub id: dbm::Id,
	pub component: dbm::Id,
	pub schema: RootSchema,
}

impl Base for ServiceType {
	const RESOURCE: &'static str = "service_type";
}

impl Resource for ServiceType {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}
