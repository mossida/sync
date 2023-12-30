use derive_more::Display;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod climate;

#[derive(Debug, Display, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    Climate,
}
