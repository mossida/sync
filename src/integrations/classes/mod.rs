use serde::{Deserialize, Serialize};

pub mod climate;
pub mod generic;

#[derive(Debug, Serialize, Deserialize)]
pub enum Class {
    Climate,
}
