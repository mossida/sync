use serde::{Deserialize, Serialize};

use crate::{Class, Entity, State};

pub type Light = Entity<LightClass, LightState>;

pub struct LightClass;

#[derive(Default, Serialize, Deserialize)]
pub enum LightState {
	Off,
	On,
	#[default]
	Unknown,
}

impl Class for LightClass {}

impl State for LightState {
	fn next(&self) -> Self {
		Self::Unknown
	}
}
