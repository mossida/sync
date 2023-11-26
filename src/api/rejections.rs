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
