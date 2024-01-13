use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::classes::climate::{HVACAction, HVACMode, Preset};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
	Off,
	SmartSchedule,
	Auto,
	Cool,
	Heat,
	Dry,
	Fan,
}

impl From<Action> for Mode {
	fn from(value: Action) -> Self {
		match value {
			Action::Heating => Mode::Heat,
			Action::Drying => Mode::Dry,
			Action::Fan => Mode::Fan,
			Action::Cooling => Mode::Cool,
			Action::HotWater => Mode::Heat,
			_ => Mode::Off,
		}
	}
}

impl From<Mode> for HVACMode {
	fn from(value: Mode) -> Self {
		match value {
			Mode::Cool => HVACMode::Cool,
			Mode::Heat => HVACMode::Heat,
			Mode::Dry => HVACMode::Dry,
			Mode::Fan => HVACMode::Fan,
			Mode::SmartSchedule => HVACMode::Auto,
			Mode::Off => HVACMode::Off,
			Mode::Auto => HVACMode::Auto,
		}
	}
}

impl From<Mode> for HVACAction {
	fn from(value: Mode) -> Self {
		match value {
			Mode::Cool => HVACAction::Cooling,
			Mode::Heat => HVACAction::Heating,
			Mode::Dry => HVACAction::Drying,
			Mode::Fan => HVACAction::Fan,
			_ => HVACAction::Idle,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Action {
	Heating,
	Drying,
	Fan,
	Cooling,
	Idle,
	Off,
	HotWater,
}

impl From<Mode> for Action {
	fn from(value: Mode) -> Self {
		match value {
			Mode::Cool => Action::Cooling,
			Mode::Heat => Action::Heating,
			Mode::Dry => Action::Drying,
			Mode::Fan => Action::Fan,
			_ => Action::Off,
		}
	}
}

impl From<Action> for HVACAction {
	fn from(value: Action) -> Self {
		match value {
			Action::Cooling => HVACAction::Cooling,
			Action::Heating => HVACAction::Heating,
			Action::Drying => HVACAction::Drying,
			Action::Fan => HVACAction::Fan,
			Action::Idle => HVACAction::Idle,
			Action::Off => HVACAction::Idle,
			Action::HotWater => HVACAction::Heating,
		}
	}
}

impl From<Action> for HVACMode {
	fn from(value: Action) -> Self {
		match value {
			Action::Cooling => HVACMode::Cool,
			Action::Heating => HVACMode::Heat,
			Action::Drying => HVACMode::Dry,
			Action::Fan => HVACMode::Fan,
			_ => HVACMode::Off,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
	pub celsius: f64,
	pub fahrenheit: f64,
	pub timestamp: String,
	#[serde(rename = "type")]
	pub r#type: String,
	pub precision: TemperatureUnit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorDataPoints {
	#[serde(rename = "insideTemperature")]
	pub inside_temperature: Temperature,
	pub humidity: Percentage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Percentage {
	#[serde(rename = "type")]
	pub r#type: String,
	pub percentage: f64,
	pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityDataPoints {
	#[serde(rename = "heatingPower")]
	pub heating_power: Option<Percentage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
	pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemperatureUnit {
	pub celsius: f64,
	pub fahrenheit: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Setting {
	// For compatibility
	#[serde(rename = "mode")]
	pub r#mode: Option<Action>,
	#[serde(rename = "type")]
	pub r#type: Option<Action>,
	pub power: String,
	pub temperature: TemperatureUnit,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TadoMode {
	Away,
	Home,
}

impl From<TadoMode> for Preset {
	fn from(value: TadoMode) -> Self {
		match value {
			TadoMode::Away => Preset::Away,
			TadoMode::Home => Preset::Home,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
	#[serde(rename = "tadoMode")]
	pub tado_mode: TadoMode,
	#[serde(rename = "geolocationOverride")]
	pub geolocation_override: bool,
	#[serde(rename = "geolocationOverrideDisableTime")]
	pub geolocation_override_disable_time: Option<bool>,
	pub preparation: Option<String>,
	pub setting: Option<Setting>,
	#[serde(rename = "overlayType")]
	pub overlay_type: Option<String>,
	pub overlay: Option<Value>,
	#[serde(rename = "openWindow")]
	pub open_window: Option<String>,
	#[serde(rename = "nextScheduleChange")]
	pub next_schedule_change: Option<String>,
	#[serde(rename = "nextTimeBlock")]
	pub next_time_block: Option<String>,
	pub link: Link,
	#[serde(rename = "activityDataPoints")]
	pub activity_data_points: ActivityDataPoints,
	#[serde(rename = "sensorDataPoints")]
	pub sensor_data_points: SensorDataPoints,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct States {
	#[serde(rename = "zoneStates")]
	pub zone_states: HashMap<String, State>,
}
