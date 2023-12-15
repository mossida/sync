use warp::Filter;

use crate::devices::handlers;

//use crate::types::ApiResponse;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let prefix = warp::path!("devices" / ..);
    prefix.and(read())
}

pub fn read() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(handlers::fetch_devices)
}
