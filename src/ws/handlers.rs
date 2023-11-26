use futures_util::StreamExt;
use log::{info, warn};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::WebSocket;

use crate::api::rejections::{Rejection, RejectionCode};
use crate::db;
use crate::entities::models::Entity;
use crate::events::models::Event;
use crate::ws::models::{Client, Clients, Message, Model};
use crate::ws::reply::{error, result};

pub async fn handle_connection(socket: WebSocket, clients: Clients) {
    let (rx, mut tx) = socket.split();
    let (send, receive) = mpsc::unbounded_channel::<Result<warp::ws::Message, warp::Error>>();

    let receive = UnboundedReceiverStream::new(receive);
    tokio::spawn(receive.forward(rx));

    let id = Uuid::new_v4();
    let client = Client { id, sender: send };

    info!("New client connected with id: ({id})");
    clients.lock().await.insert(id, client);

    while let Some(result) = tx.next().await {
        let message = result.unwrap();

        if !message.is_close() {
            handle_message(message, clients.lock().await.get(&id).unwrap()).await;
        }
    }

    info!("Client ({id}) has disconnected");
    clients.lock().await.remove(&id);
}

pub async fn handle_message(response: warp::ws::Message, client: &Client) {
    match serde_json::from_str::<Message>(response.to_str().unwrap()) {
        Ok(message) => match message {
            Message::FETCH { model } => {
                handle_fetch(model, client).await;
            }
            _ => {
                error(
                    client,
                    Rejection {
                        reason: RejectionCode::INTERFACE,
                        message: "This type of message is not accepted".to_string(),
                    },
                );
            }
        },
        Err(err) => {
            let err_message = err.to_string();
            warn!("Tried to parse message with error: {err_message}");
        }
    };
}

pub async fn handle_fetch(model: Model, client: &Client) {
    match model {
        Model::ENTITY => {
            let mut response = db::get().query("SELECT * FROM entity").await.unwrap();
            let list = response.take::<Vec<Entity>>(0).unwrap();
            result(client, list);
        }
        Model::EVENT => {
            let mut response = db::get().query("SELECT * FROM event").await.unwrap();
            let list = response.take::<Vec<Event>>(0).unwrap();
            result(client, list);
        }
        Model::STATE => {}
    }
}
