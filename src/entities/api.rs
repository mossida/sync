use warp::Filter;

use crate::entities::handlers;

//use crate::types::ApiResponse;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let prefix = warp::path!("entities" / ..);
    prefix.and(create().or(read()).or(delete()))
}

pub fn create() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and_then(handlers::create_entity)
}

pub fn read() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(handlers::fetch_entities)
}

pub fn delete() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!(String)
        .and(warp::delete())
        .and_then(handlers::delete_entity)
}
