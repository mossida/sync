use serde::{Deserialize, Serialize};

use super::expose::Expose;

#[derive(Debug, Serialize, Deserialize)]
pub struct Devices(pub Vec<Device>);

// NOTE: Unstable, needs to matched with original schema
#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	ieee_address: String,
	r#type: String,
	network_address: i128,
	supported: bool,
	disabled: bool,
	friendly_name: String,
	description: Option<String>,
	definition: Option<Definition>,
	power_source: Option<String>,
	date_code: Option<String>,
	model_id: Option<String>,
	scenes: Option<Vec<()>>,
	interviewing: bool,
	interview_completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Definition {
	vendor: String,
	model: String,
	description: String,
	supports_ota: bool,

	#[serde(default)]
	exposes: Vec<Expose>,

	#[serde(default)]
	options: Vec<Expose>,
}
