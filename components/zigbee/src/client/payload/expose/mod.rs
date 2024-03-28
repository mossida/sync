use serde::{Deserialize, Serialize};

use self::generic::{Binary, Composite, Enum, Numeric, Text};

pub mod generic;
pub mod specific;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expose {
	Generic(Generic),
	Specific(Specific),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Generic {
	Binary(Binary),
	Numeric(Numeric),
	Enum(Enum),
	Text(Text),
	Composite(Composite),
	List,
}

// Requires validation with original schema
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Specific {
	Light,
	Switch,
	Fan,
	Cover,
	Lock,
	Climate,
}
