use axum::extract::ws::WebSocket;
use axum::extract::WebSocketUpgrade;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use tower_http::request_id::RequestId;

use crate::rpc::client::Client;

pub(super) fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/rpc", get(handler))
}

async fn handler(ws: WebSocketUpgrade, Extension(id): Extension<RequestId>) -> impl IntoResponse {
    ws
        // Set the maximum frame size
        //.max_frame_size(*cnf::WEBSOCKET_MAX_FRAME_SIZE)
        // Set the maximum message size
        //.max_message_size(*cnf::WEBSOCKET_MAX_MESSAGE_SIZE)
        // Set the potential WebSocket protocol formats
        .protocols(["json"])
        // Handle the WebSocket upgrade and process messages
        .on_upgrade(move |socket| handle_socket(socket, id))
}

async fn handle_socket(ws: WebSocket, id: RequestId) {
    let rpc = Client::new(id);
    Client::serve(&rpc, ws).await;
}
