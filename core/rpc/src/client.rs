use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use dashmap::DashSet;
use dbm::Id;
use futures::StreamExt;
use ractor::{
	concurrency::Duration,
	factory::{DeadMansSwitchConfiguration, Factory, FactoryMessage, Job, RoutingMode},
	Actor, ActorRef,
};
use tokio::{select, sync::mpsc::channel};
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;
use tower_http::request_id::RequestId;
use tracing::error;

use crate::{
	request::Request,
	worker::{Worker, WorkerKey},
};

pub struct Client {
	id: RequestId,
	ct: CancellationToken,
}

impl Client {
	pub fn new(id: RequestId) -> Self {
		Self {
			id,
			ct: CancellationToken::new(),
		}
	}

	pub fn handle_message(
		&self,
		message: Message,
		factory: ActorRef<FactoryMessage<WorkerKey, Request>>,
	) -> Result<(), String> {
		match message {
			Message::Text(text) => {
				let request: Request =
					serde_json::from_str(text.as_str()).map_err(|e| e.to_string())?;

				factory
					.send_message(FactoryMessage::Dispatch(Job {
						key: request.id,
						msg: request,
						options: Default::default(),
					}))
					.map_err(|e| e.to_string())?;
			}
			Message::Close(_) => {
				self.ct.cancel();
			}
			_ => { /* Ignore */ }
		}

		Ok(())
	}

	pub async fn serve(&self, socket: WebSocket) -> Result<(), err::Error> {
		// Create the forwarding system from the socket and to the socket
		let (sender, mut receiver) = socket.split();
		let (internal_sender, internal_receiver) = channel::<Message>(24);
		let forwarder =
			tokio::spawn(ReceiverStream::new(internal_receiver).map(Ok).forward(sender));

		// Get the client ID
		let id = Id::rand().to_raw();
		let client_id = self.id.header_value().to_str().unwrap_or(id.as_str());

		// Spawn the factory that is going to handle the requests
		let (factory, handle) = Actor::spawn(
			Some(client_id.to_string()),
			Factory {
				worker_count: 3,
				worker_parallel_capacity: 1,
				routing_mode: RoutingMode::<WorkerKey>::Queuer, // First worker available
				discard_threshold: Some(50),                    // Maximum queue size
				dead_mans_switch: Some(DeadMansSwitchConfiguration {
					detection_timeout: Duration::from_secs(5),
					kill_worker: true,
				}),
				..Default::default()
			},
			Box::new(Worker {
				sender: Arc::new(internal_sender),
				used_ids: Arc::new(DashSet::new()),
			}),
		)
		.await?;

		// Loop though the messages and handle them sending them to the factory
		loop {
			select! {
				biased;
				_ = self.ct.cancelled() => break,
				Some(request) = receiver.next() => match request {
						Ok(message) => {
							let result = self.handle_message(message, factory.clone());

							if let Err(e) = result {
								error!("WebSocket: Error handling message: {}", e);
							}
						},
						Err(err) => {
							error!("WebSocket: Error receiving message from client: {}", err);
							self.ct.cancel();
						}
					}
			}
		}

		// Shutdown the factory and wait for it to finish
		forwarder.abort();
		factory.stop(None);
		let _ = handle.await;

		Ok(())
	}
}
