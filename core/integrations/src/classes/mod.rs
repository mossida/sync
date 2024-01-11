use derive_more::Display;
use serde::{Deserialize, Serialize};

pub mod climate;
mod sensor;

#[derive(Debug, Display, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    Climate,
}
