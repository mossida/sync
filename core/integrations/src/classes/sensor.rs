use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StateClass {
	Measurement,
	Total,
	TotalIncreasing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Type {
	Numeric,
	Date,
	Enum,
	Timestamp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Attribute {
	SensorType(Type),
	StateClass(StateClass),
	LastReset(DateTime<Utc>),
}
