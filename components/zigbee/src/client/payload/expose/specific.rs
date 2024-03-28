use serde::{Deserialize, Serialize};

use super::Generic;

#[derive(Debug, Serialize, Deserialize)]
pub struct Specific {
	#[serde(default)]
	features: Vec<Generic>,
}
