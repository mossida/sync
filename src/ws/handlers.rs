use futures_util::StreamExt;
use ractor::Actor;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;

use crate::api::rejections::{Rejection, RejectionCode};
use crate::ws::actors::ClientActor;
use crate::ws::models::MessageHandler;
use crate::ws::reply::error;

pub async fn handle_connection(socket: WebSocket) {
    let (rx, mut tx) = socket.split();
    let (send, receive) = mpsc::unbounded_channel::<warp::ws::Message>();

    let receive = UnboundedReceiverStream::new(receive);
    tokio::spawn(receive.map(Ok).forward(rx));

    let _ = (|| async move {
        let (actor, actor_handle) = Actor::spawn(None, ClientActor, send.clone()).await?;

        while let Some(result) = tx.next().await {
            let message = result?;

            if !message.is_close() && message.is_text() {
                let handler_result =
                    serde_json::from_str::<Box<dyn MessageHandler>>(message.to_str().unwrap());

                match handler_result {
                    Ok(handler) => actor.cast(handler)?,
                    Err(err) => error(
                        0,
                        &send,
                        Rejection {
                            reason: RejectionCode::INTERFACE,
                            message: err.to_string(),
                        },
                    ),
                }
            }
        }

        actor.stop(None);
        actor_handle.await?;

        Ok::<(), Box<dyn std::error::Error>>(())
    })()
    .await;

    // If the actor cannot correctly be handled close the connection and ignore errors.
}
