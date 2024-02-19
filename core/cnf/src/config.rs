use rumqttd::Config as Mqtt;
use serde::Deserialize;

use crate::{database::Database, secrets::Secrets};

#[derive(Deserialize)]
pub struct Configuration {
	pub database: Database,
	pub secrets: Secrets,
	pub mqtt: Mqtt,
}
