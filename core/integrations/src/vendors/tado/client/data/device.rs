use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Characteristics {
	pub capabilities: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionState {
	pub value: bool,
	pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
	#[serde(rename = "deviceType")]
	pub device_type: String,
	#[serde(rename = "serialNo")]
	pub serial_no: String,
	#[serde(rename = "shortSerialNo")]
	pub short_serial_no: String,
	#[serde(rename = "currentFwVersion")]
	pub current_fw_version: String,
	#[serde(rename = "connectionState")]
	pub connection_state: ConnectionState,
	pub characteristics: Characteristics,
	#[serde(rename = "inPairingMode")]
	pub in_pairing_mode: Option<bool>,
	#[serde(rename = "batteryState")]
	pub battery_state: Option<String>,
	pub duties: Option<Vec<String>>,
}
