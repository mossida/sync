use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TemperatureInfo {
	pub min: f64,
	pub max: f64,
	pub step: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Temperatures {
	pub celsius: TemperatureInfo,
	pub fahrenheit: TemperatureInfo,
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapabilityType {
	Heating,
	AirConditioning,
	HotWater,
}

#[derive(Serialize, Deserialize)]
pub struct Capability {
	#[serde(rename = "type")]
	pub r#type: CapabilityType,
	pub temperatures: Temperatures,
}
