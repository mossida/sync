use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use typetag::serde;
use warp::reject::Reject;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RejectionCode {
    UNKNOWN,
    DATABASE,
    INTERFACE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rejection {
    pub reason: RejectionCode,
    pub message: String,
}

impl Reject for Rejection {}

impl Display for Rejection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for Rejection {}

impl From<surrealdb::Error> for Rejection {
    fn from(value: surrealdb::Error) -> Self {
        Rejection {
            reason: RejectionCode::DATABASE,
            message: value.to_string(),
        }
    }
}
