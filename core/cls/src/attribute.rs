use dashmap::DashSet;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::hash::{Hash, Hasher};

// TODO: Understand if this is the correct way to implement attributes
pub type Attributes = DashSet<Attribute>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Attribute {
	pub name: String,
	pub value: Value,
}

impl Attribute {
	pub fn new<S, V>(name: S, value: V) -> Self
	where
		S: Into<String>,
		V: Into<Value>,
	{
		Attribute {
			name: name.into(),
			value: value.into(),
		}
	}
}

impl Hash for Attribute {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}
