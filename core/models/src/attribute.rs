use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Attributes are a collection of key-value pairs
/// that can be build from a dynamic list of enum variants carrying
/// a custom value
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Attributes(HashMap<String, Value>);

impl Attributes {
	pub fn new() -> Attributes {
		Attributes(Default::default())
	}
}

impl<T> From<Vec<T>> for Attributes
where
	T: Serialize,
{
	fn from(value: Vec<T>) -> Self {
		// TODO: Optimize this conversion
		let object = serde_json::to_value(value).unwrap_or(Value::Array(vec![]));
		let items: &Vec<Value> = object.as_array().unwrap(); // This is safe

		let map: HashMap<String, Value> = items.iter().fold(HashMap::new(), |mut acc, item| {
			if let Value::Object(object) = item {
				acc.extend(object.to_owned());
			}
			acc
		});

		Attributes(map)
	}
}
