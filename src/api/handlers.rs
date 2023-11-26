use rkyv::bytecheck::Error;
use std::convert::Infallible;

use warp::http::StatusCode;

use crate::api::rejections::{Rejection, RejectionCode};

pub fn handle_db_error(error: surrealdb::Error) -> warp::Rejection {
    warp::reject::custom(Rejection {
        code: RejectionCode::DATABASE,
        message: error.as_error().to_string(),
    })
}

pub async fn handle_rejection(r: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    dbg!(&r);

    let custom = r.find::<Rejection>();

    match custom {
        None => Ok(warp::reply::with_status(
            warp::reply::json(&Rejection {
                code: RejectionCode::UNKNOWN,
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
