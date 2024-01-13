use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::vendors::tado::client::data::states::{Percentage, Temperature};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherState {
	#[serde(rename = "type")]
	pub r#type: String,
	pub value: String,
	pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
	#[serde(rename = "solarIntensity")]
	pub solar_intensity: Percentage,
	#[serde(rename = "outsideTemperature")]
	pub outside_temperature: Temperature,
	#[serde(rename = "weatherState")]
	pub weather_state: WeatherState,
}
