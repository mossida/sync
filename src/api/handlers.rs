use std::convert::Infallible;

use warp::http::StatusCode;

use crate::api::rejections::{Rejection, RejectionCode};

pub async fn handle_rejection(r: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let custom = r.find::<Rejection>();

    match custom {
        None => Ok(warp::reply::with_status(
            warp::reply::json(&Rejection {
                reason: RejectionCode::UNKNOWN,
                message: String::from("Error is not defined"),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
        Some(c) => Ok(warp::reply::with_status(
            warp::reply::json(&c),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
