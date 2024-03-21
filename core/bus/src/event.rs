use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Event {
	Start,
	Stop,
	VendorStart(String),
	ManualTrigger(String),

	// System events
	Time,
}
