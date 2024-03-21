use std::hash::{DefaultHasher, Hash, Hasher};

use dbm::{
	resource::{Base, Resource},
	Id, IdKind,
};
use err::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct ServiceData {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceType {
	pub id: dbm::Id,
	#[serde(flatten)]
	pub data: ServiceData,
}

impl Base for ServiceType {
	const RESOURCE: &'static str = "service_type";
}

impl Resource for ServiceType {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

impl ServiceType {
	pub async fn register(data: ServiceData) -> Result<Vec<Self>, err::Error> {
		let mut hasher = DefaultHasher::new();
		data.hash(&mut hasher);

		let id: Id = IdKind::Record(hasher.finish().into()).into();
		let service_type = ServiceType {
			id,
			data,
		};

		service_type.create().await
	}
}
