use std::convert::Infallible;
use warp::ws::Ws;
use warp::{Filter, Reply};

use crate::ws::handlers::handle_connection;
use crate::ws::{with_clients, Clients};

async fn convert_connection(ws: Ws, clients: Clients) -> Result<impl Reply, Infallible> {
    Ok(ws.on_upgrade(move |socket| handle_connection(socket, clients)))
}

pub fn route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ws")
        .and(warp::ws())
        .and(with_clients())
        .and_then(convert_connection)
}
