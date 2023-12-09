use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    pub heating_power: Percentage,
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
    #[serde(rename = "type")]
    pub r#type: String,
    pub power: String,
    pub temperature: TemperatureUnit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    #[serde(rename = "tadoMode")]
    pub tado_mode: String,
    #[serde(rename = "geolocationOverride")]
    pub geolocation_override: bool,
    #[serde(rename = "geolocationOverrideDisableTime")]
    pub geolocation_override_disable_time: Option<bool>,
    pub preparation: Option<String>,
    pub setting: Setting,
    #[serde(rename = "overlayType")]
    pub overlay_type: Option<String>,
    pub overlay: Option<String>,
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
