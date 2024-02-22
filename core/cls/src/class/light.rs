use serde::{Deserialize, Serialize};

use crate::{Class, Entity, State};

pub type Light = Entity<LightClass, LightState>;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename = "snake_case")]
pub struct LightClass;

#[derive(Default, Serialize, Deserialize)]
pub enum LightState {
	Off,
	On,
	#[default]
	Unknown,
}

impl Class for LightClass {
	const NAME: &'static str = "light";
}

impl State for LightState {
	fn next(&self) -> Self {
		Self::Unknown
	}
}
