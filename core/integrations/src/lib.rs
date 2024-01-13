use revision::revisioned;
use serde::{Deserialize, Serialize};

use macros::VendorBuilder;

pub mod classes;
pub mod scheduler;
pub mod vendors;

#[derive(VendorBuilder, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[revisioned(revision = 1)]
pub enum Vendor {
	Tado,
}
