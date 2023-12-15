use warp::Filter;

use crate::events::handlers;

pub fn routes() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let prefix = warp::path!("events" / ..);
    prefix.and(read())
}

pub fn read() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(handlers::fetch_events)
}
