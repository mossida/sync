use std::convert::Infallible;

use surreal_id::NewId;
use warp::http::StatusCode;

use crate::entities::api::{delete, fetch_all};
use crate::entities::models::EntityId;
use crate::errors::Error;

pub async fn create_entity() -> miette::Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("Test"))
}

pub async fn fetch_entities() -> miette::Result<impl warp::Reply, warp::Rejection> {
    let list = fetch_all().await?;
    Ok(warp::reply::json(&list))
}

pub async fn delete_entity(id: String) -> miette::Result<impl warp::Reply, warp::Rejection> {
    let _ = delete(EntityId::new(id).map_err(Error::from)?).await?;
    Ok(StatusCode::ACCEPTED)
}
