use std::convert::Infallible;

use surrealdb::sql::Uuid;
use warp::http::StatusCode;

use crate::api::rejections::{Rejection, RejectionCode};
use crate::entities::api::{delete, fetch};

pub async fn create_entity() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("test"))
}

pub async fn fetch_entities() -> Result<impl warp::Reply, warp::Rejection> {
    let list = fetch().await.map_err(warp::reject::custom)?;
    Ok(warp::reply::json(&list))
}

pub async fn delete_entity(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = delete(Uuid::try_from(id).map_err(|_| {
        warp::reject::custom(Rejection {
            reason: RejectionCode::INTERFACE,
            message: "Provided id is not valid".to_string(),
        })
    })?)
    .await?;
    Ok(StatusCode::ACCEPTED)
}
