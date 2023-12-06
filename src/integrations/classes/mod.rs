use serde::{Deserialize, Serialize};

pub mod climate;

#[derive(Debug, Serialize, Deserialize)]
pub enum Class {
    Climate,
}
