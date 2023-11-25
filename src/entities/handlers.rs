use std::convert::Infallible;

use warp::http::StatusCode;
use warp::Rejection;

use crate::db;
use crate::entities::models::Entity;
use crate::helpers::Helper;

pub async fn create_entity() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("test"))
}

pub async fn fetch_entities() -> Result<impl warp::Reply, Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM entity")
        .await
        .map_err(Helper::reject_db)?;

    response
        .take::<Vec<Entity>>(0)
        .map_err(Helper::reject_db)
        .map(|result| warp::reply::json(&result))
}

pub async fn delete_entity(id: String) -> Result<impl warp::Reply, Rejection> {
    let _: Option<Entity> = db::get()
        .delete(("entity", id))
        .await
        .map_err(Helper::reject_db)?;

    Ok(StatusCode::ACCEPTED)
}
