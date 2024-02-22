use futures::{future, Stream, StreamExt};
use ractor::ActorRef;
use tokio::{select, sync::broadcast};
use tokio_util::sync::CancellationToken;
use tracing::error;

use crate::Event;

pub struct Bus {
	sender: broadcast::Sender<Event>,
	receiver: broadcast::Receiver<Event>,
}

impl Bus {
	pub fn new() -> Self {
		let (sender, receiver) = broadcast::channel::<Event>(1000);

		Bus {
			sender,
			receiver,
		}
	}

	pub fn emit(&self, event: Event) -> usize {
		match self.sender.send(event) {
			Ok(items) => items,
			Err(_) => {
				error!("Bus failed to emit event");
				0
			}
		}
	}

	pub fn publish<S>(&self, stream: S) -> CancellationToken
	where
		S: Stream<Item = Event> + Send + 'static,
	{
		let stream = Box::pin(stream);

		let token = CancellationToken::new();
		let inner_token = token.child_token();
		let sender = self.sender.clone();

		let _ = tokio::spawn(async move {
			select! {
				biased;
				_ = inner_token.cancelled() => {},
				_ = stream.for_each(|e| {
					let _ = sender.send(e);
					future::ready(())
				}) => {}
			}
		});

		token
	}

	pub fn subscribe(&self) -> broadcast::Receiver<Event> {
		self.sender.subscribe()
	}

	pub fn subscribe_actor<T>(&self, cell: ActorRef<T>) -> CancellationToken
	where
		T: From<Event> + Send + Sync + 'static,
	{
		let mut receiver = self.receiver.resubscribe();
		let token = CancellationToken::new();
		let inner_token = token.child_token();

		let _ = tokio::spawn(async move {
			select! {
				_ = inner_token.cancelled() => {},
				Ok(event) = receiver.recv() => {
					let result = cell.send_message(event.into());
					if result.is_err() {
						error!("Failed to send message to actor");
						inner_token.cancel();
					}
				}
			}
		});

		token
	}
}
