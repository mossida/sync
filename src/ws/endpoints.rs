use std::convert::Infallible;

use warp::ws::Ws;
use warp::{Filter, Reply};

use crate::ws::handlers::handle_connection;

async fn convert_connection(ws: Ws) -> Result<impl Reply, Infallible> {
    Ok(ws.on_upgrade(handle_connection))
}

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ws")
        .and(warp::ws())
        .and_then(convert_connection)
}
