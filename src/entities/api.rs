use crate::entities::handlers;
use warp::Filter;
//use crate::types::ApiResponse;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    create().or(read()).or(delete())
}

pub fn create() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("entity")
        .and(warp::post())
        .and_then(handlers::create_entity)
}

pub fn read() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("entity")
        .and(warp::get())
        .and_then(handlers::fetch_entities)
}

pub fn delete() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("entity" / String)
        .and(warp::delete())
        .and_then(handlers::delete_entity)
}
