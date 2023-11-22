use crate::db;
use crate::entities::models::Entity;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn create_entity() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("test"))
}

pub async fn fetch_entities() -> Result<impl warp::Reply, Infallible> {
    let result = db::get().query("SELECT * FROM entity").await;
    let list: Result<Vec<Entity>, surrealdb::Error> = result.unwrap().take(0);

    Ok(warp::reply::json(&list.unwrap()))
}

pub async fn delete_entity(id: String) -> Result<impl warp::Reply, Infallible> {
    let _: Result<Option<Entity>, surrealdb::Error> = db::get().delete(("entity", id)).await;

    Ok(StatusCode::ACCEPTED)
}
