use std::convert::Infallible;

use serde_json::json;
use warp::http::StatusCode;

use crate::errors::Error;

pub async fn handle_rejection(r: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    let custom = r.find::<Error>();

    match custom {
        None => Ok(warp::reply::with_status(
            warp::reply::json(&json!({ "error": "Unknown error" })),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
        Some(c) => Ok(warp::reply::with_status(
            warp::reply::json(&c),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}
