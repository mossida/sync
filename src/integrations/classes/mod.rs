use serde::{Deserialize, Serialize};

pub mod climate;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    Climate,
}
