use serde::Serialize;
use serde_repr::Serialize_repr;
use warp::reject::Reject;

#[derive(Serialize_repr, Debug)]
#[repr(u16)]
pub enum RejectionCode {
    UNKNOWN = 0,
    DATABASE = 500,
}

#[derive(Serialize, Debug)]
pub struct Rejection {
    pub code: RejectionCode,
    pub message: String,
}

impl Reject for Rejection {}
