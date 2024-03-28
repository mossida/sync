use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Generic;

#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
	pub name: String,
	pub label: String,
	pub property: String,
	pub access: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Binary {
	pub value_on: Value,
	pub value_off: Value,
	pub value_toggle: Option<Value>,

	#[serde(flatten)]
	pub property: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Numeric {
	// pub presets: Value
	pub value_min: Option<f64>,
	pub value_max: Option<f64>,
	pub value_step: Option<f64>,
	pub unit: Option<String>,

	#[serde(flatten)]
	pub property: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
	pub values: Vec<Value>,

	#[serde(flatten)]
	pub property: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
	#[serde(flatten)]
	pub property: Property,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Composite {
	features: Vec<Generic>,

	#[serde(flatten)]
	pub property: Property,
}
