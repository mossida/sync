use std::time::Duration;

use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use ractor::factory::{DeadMansSwitchConfiguration, Factory, FactoryMessage, Job, RoutingMode};
use ractor::Actor;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;
use tower_http::request_id::RequestId;
use tracing::{debug, error, trace};
use uuid::Uuid;

use crate::rpc::worker::{WorkerBuilder, WorkerKey};

pub struct Client {
    id: RequestId,
    canceller: CancellationToken,
}

impl Client {
    pub fn new(id: RequestId) -> Self {
        Self {
            id,
            canceller: CancellationToken::new(),
        }
    }

    pub async fn serve(&self, socket: WebSocket) {
        let (sender, mut receiver) = socket.split();
        let (internal_sender, internal_receiver) = tokio::sync::mpsc::channel::<Message>(24);
        let forwarder = tokio::spawn(
            ReceiverStream::new(internal_receiver)
                .map(Ok)
                .forward(sender),
        );

        let client_id = self.id.header_value().to_str().unwrap();

        trace!("WebSocket: Client {} connected", client_id);

        let factory = Actor::spawn(
            Some(client_id.to_string()),
            Factory {
                worker_count: 6,
                worker_parallel_capacity: 1,
                routing_mode: RoutingMode::<WorkerKey>::Queuer, // First worker available
                discard_threshold: Some(50),                    // Maximum queue size
                dead_mans_switch: Some(DeadMansSwitchConfiguration {
                    detection_timeout: Duration::from_secs(5),
                    kill_worker: true,
                }),
                ..Default::default()
            },
            Box::new(WorkerBuilder {
                sender: internal_sender,
                canceller: self.canceller.clone(),
            }),
        )
        .await;

        if let Err(err) = factory {
            error!("WebSocket: Cannot spawn dispatcher: {}", err);
            self.canceller.cancel();
        } else if let Ok((dispatcher, handle)) = factory {
            loop {
                tokio::select! {
                    biased;
                    _ = self.canceller.cancelled() => break,
                    Some(request) = receiver.next() => match request {
                        Ok(message) => {
                            if let Err(err) = dispatcher.send_message(FactoryMessage::Dispatch(Job {
                                key: Uuid::new_v4(),
                                msg: message,
                                options: Default::default(),
                            })) {
                                // Factory cannot handle messages correctly
                                debug!("WebSocket: Cannot handle RPC request: {}", err);
                                self.canceller.cancel();
                            }
                        }
                        Err(err) => {
                            error!("WebSocket: Error receiving message from client: {}", err);
                            self.canceller.cancel();
                        }
                    }
                }
            }

            trace!("WebSocket: Client {} disconnected", client_id);

            forwarder.abort();
            dispatcher.stop(None);
            let _ = handle.await;
        }
    }
}
