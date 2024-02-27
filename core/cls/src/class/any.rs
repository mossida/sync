use serde_json::Value;

use crate::{Class, Entity, State};

pub type Any = Entity<AnyClass, Value>;

pub struct AnyClass;

impl Class for AnyClass {
	const NAME: &'static str = "any";
}

impl State for Value {
	fn next(&self) -> Self {
		self.clone()
	}
}
