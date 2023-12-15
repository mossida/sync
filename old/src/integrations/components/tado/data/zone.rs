use serde::{Deserialize, Serialize};

use crate::integrations::components::tado::data::device::Device;

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenWindowDetection {
    pub supported: bool,
    pub enabled: bool,
    #[serde(rename = "timeoutInSeconds")]
    pub timeout_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DazzleMode {
    pub supported: bool,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Zone {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "deviceTypes")]
    pub device_types: Vec<String>,
    pub devices: Vec<Device>,
    #[serde(rename = "reportAvailable")]
    pub report_available: bool,
    #[serde(rename = "showScheduleSetup")]
    pub show_schedule_setup: bool,
    #[serde(rename = "supportsDazzle")]
    pub supports_dazzle: bool,
    #[serde(rename = "dazzleEnabled")]
    pub dazzle_enabled: bool,
    #[serde(rename = "dazzleMode")]
    pub dazzle_mode: DazzleMode,
    #[serde(rename = "openWindowDetection")]
    pub open_window_detection: OpenWindowDetection,
}
