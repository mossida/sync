use std::collections::HashMap;
use std::convert::Infallible;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use warp::Filter;

use crate::types::SyncObject;
use crate::ws::models::Clients;

pub mod api;
mod handlers;
mod models;
mod reply;

static WS_CLIENTS: Lazy<Clients> = Lazy::new(|| SyncObject::new(Mutex::new(HashMap::new())));

pub fn with_clients() -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || WS_CLIENTS.clone())
}
