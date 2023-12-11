use std::convert::Infallible;

use serde_json::Value;
use surreal_id::NewId;
use warp::http::StatusCode;

use crate::api::rejections::Rejection;
use crate::entities::api::{delete, fetch};
use crate::entities::models::EntityId;

pub async fn create_entity() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("Test"))
}

pub async fn fetch_entities() -> Result<impl warp::Reply, warp::Rejection> {
    let list = fetch().await.map_err(warp::reject::custom)?;
    Ok(warp::reply::json(&list))
}

pub async fn delete_entity(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = delete(EntityId::new(id).map_err(|err| Rejection::from(err))?).await?;
    Ok(StatusCode::ACCEPTED)
}
