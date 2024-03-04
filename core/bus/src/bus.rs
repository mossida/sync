use std::marker::PhantomData;

use futures::{future, Stream, StreamExt};
use ractor::ActorRef;
use tokio::{select, sync::broadcast};
use tokio_util::sync::CancellationToken;
use tracing::error;

/// A generic message bus that allows emitting, publishing, and subscribing to events of type `T`.
pub struct Bus<T> {
	sender: broadcast::Sender<T>,
	receiver: broadcast::Receiver<T>,
	_marker: PhantomData<T>,
}

impl<T> Default for Bus<T>
where
	T: Clone + Send + Sync + 'static,
{
	/// Creates a new `Bus` with the default settings.
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Bus<T>
where
	T: Clone + Send + Sync + 'static,
{
	/// Creates a new `Bus` with the specified capacity.
	///
	/// # Returns
	///
	/// A new `Bus` instance.
	pub fn new() -> Self {
		let (sender, receiver) = broadcast::channel::<T>(1000);

		Bus {
			sender,
			receiver,
			_marker: PhantomData,
		}
	}

	/// Emits an event to all subscribers of the `Bus`.
	///
	/// # Arguments
	///
	/// * `event` - The event to emit.
	///
	/// # Returns
	///
	/// The number of subscribers that received the event.
	///
	/// # Examples
	///
	/// ```
	/// let bus = Bus::new();
	/// let event = "Hello, world!";
	/// let num_subscribers = bus.emit(event);
	/// println!("Event emitted to {} subscribers", num_subscribers);
	/// ```
	pub fn emit(&self, event: T) -> usize {
		match self.sender.send(event) {
			Ok(items) => items,
			Err(_) => {
				error!("Bus failed to emit event");
				0
			}
		}
	}

	/// Publishes a stream of events to the `Bus`.
	///
	/// # Arguments
	///
	/// * `stream` - The stream of events to publish.
	///
	/// # Returns
	///
	/// A `CancellationToken` that can be used to cancel the publishing operation.
	///
	/// # Examples
	///
	/// ```
	/// use futures::stream::StreamExt;
	///
	/// let bus = Bus::new();
	/// let stream = futures::stream::iter(vec![1, 2, 3]);
	/// let token = bus.publish(stream);
	/// // ... do some work ...
	/// token.cancel();
	/// ```
	pub fn publish<S>(&self, stream: S) -> CancellationToken
	where
		S: Stream<Item = T> + Send + 'static,
	{
		let stream = Box::pin(stream);

		let token = CancellationToken::new();
		let inner_token = token.child_token();
		let sender = self.sender.clone();

		tokio::spawn(async move {
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

	/// Subscribes to the `Bus` and returns a receiver for receiving events.
	///
	/// # Returns
	///
	/// A `broadcast::Receiver` that can be used to receive events from the `Bus`.
	///
	/// # Examples
	///
	/// ```
	/// let bus = Bus::new();
	/// let receiver = bus.subscribe();
	/// // ... receive events from the receiver ...
	/// ```
	pub fn subscribe(&self) -> broadcast::Receiver<T> {
		self.sender.subscribe()
	}

	/// Subscribes an actor to the `Bus` and returns a `CancellationToken` that can be used to cancel the subscription.
	///
	/// # Arguments
	///
	/// * `cell` - The `ActorRef` of the actor to subscribe.
	///
	/// # Returns
	///
	/// A `CancellationToken` that can be used to cancel the subscription.
	///
	/// # Examples
	///
	/// ```
	/// use ractor::ActorRef;
	///
	/// let bus = Bus::new();
	/// let actor = MyActor::new();
	/// let cell = actor.cell();
	/// let token = bus.subscribe_actor(cell);
	/// // ... do some work ...
	/// token.cancel();
	/// ```
	pub fn subscribe_actor<M>(&self, cell: ActorRef<M>) -> CancellationToken
	where
		M: From<T> + Send + Sync + 'static,
	{
		let mut receiver = self.receiver.resubscribe();
		let token = CancellationToken::new();
		let inner_token = token.child_token();

		tokio::spawn(async move {
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
