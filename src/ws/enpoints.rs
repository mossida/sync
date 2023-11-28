use std::convert::Infallible;

use warp::{Filter, Reply};
use warp::ws::Ws;

use crate::ws::{Clients, with_clients};
use crate::ws::handlers::handle_connection;

async fn convert_connection(ws: Ws, clients: Clients) -> Result<impl Reply, Infallible> {
    Ok(ws.on_upgrade(move |socket| handle_connection(socket, clients)))
}

pub fn route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ws")
        .and(warp::ws())
        .and(with_clients())
        .and_then(convert_connection)
}
