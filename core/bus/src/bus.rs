use futures::{future, Stream, StreamExt};
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
		let (sender, receiver) = broadcast::channel::<Event>(10);

		Bus {
			sender,
			receiver,
		}
	}

	pub fn send(&self, event: Event) -> usize {
		match self.sender.send(event) {
			Ok(items) => items,
			Err(_) => {
				error!("Bus failed to send event");
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
}
