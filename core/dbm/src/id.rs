use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id as RecordId, Thing};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Kind {
	Record(RecordId),
	Thing(Thing),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(from = "Kind")]
pub struct Id(RecordId);

impl Id {
	pub fn new() -> Self {
		Self::rand()
	}

	pub fn rand() -> Self {
		Self(RecordId::rand())
	}

	pub fn to_raw(&self) -> String {
		self.0.to_raw()
	}

	pub fn to_thing(&self, resource: &str) -> Thing {
		Thing {
			tb: resource.to_owned(),
			id: self.0.to_owned(),
		}
	}
}

impl Default for Id {
	fn default() -> Self {
		Self::new()
	}
}

impl From<Kind> for Id {
	fn from(value: Kind) -> Self {
		match value {
			Kind::Record(id) => Self(id),
			Kind::Thing(thing) => Self(thing.id),
		}
	}
}

impl From<Id> for RecordId {
	fn from(value: Id) -> Self {
		value.0
	}
}

impl Serialize for Id {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match &self.0 {
			RecordId::Number(n) => n.serialize(serializer),
			RecordId::String(s) => s.serialize(serializer),
			RecordId::Array(a) => a.serialize(serializer),
			RecordId::Object(o) => o.serialize(serializer),
			RecordId::Generate(g) => g.serialize(serializer),
		}
	}
}
