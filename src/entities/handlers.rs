use std::convert::Infallible;

use warp::http::StatusCode;

use crate::api::handlers::handle_db_error;
use crate::db;
use crate::entities::models::Entity;

pub async fn create_entity() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("test"))
}

pub async fn fetch_entities() -> Result<impl warp::Reply, warp::Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM entity")
        .await
        .map_err(handle_db_error)?;

    let list = response.take::<Vec<Entity>>(0).map_err(handle_db_error)?;

    Ok(warp::reply::json(&list))
}

pub async fn delete_entity(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let _: Option<Entity> = db::get()
        .delete(("entity", id))
        .await
        .map_err(handle_db_error)?;

    Ok(StatusCode::ACCEPTED)
}
