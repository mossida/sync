use futures::Stream;

use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tokio_util::sync::CancellationToken;
use tracing::error;

/// A generic message bus that allows emitting, publishing, and subscribing to events of type `T`.
pub struct Bus<T> {
	state: broadcast::Sender<T>,
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
	// TODO: Understand if any better channel exists

	/// Creates a new `Bus` with the specified capacity.
	///
	/// # Returns
	///
	/// A new `Bus` instance.
	pub fn new() -> Self {
		// TODO: Understand how much capacity is needed
		let (sender, _) = broadcast::channel::<T>(1000);

		Bus {
			state: sender,
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
		match self.state.send(event) {
			Ok(items) => items,
			Err(e) => {
				// Happens if there is no body to receive the message
				error!("Bus failed to emit event: ${}", e);
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
		let token = CancellationToken::new();
		let inner_token = token.child_token();
		let sender = self.state.clone();

		// TODO: Understand if needs to continue to publish after an error
		// TODO: Understand if a task is efficient to publish
		tokio::spawn(async move {
			tokio::pin!(stream);

			tokio::select! {
				_ = inner_token.cancelled() => {},
				has_error = stream.all(|item| sender.send(item).is_ok()) => {
					if has_error {
						error!("Stream will not be consumed anymore due to an error");
						inner_token.cancel();
					}
				}
			}
		});

		token
	}

	/// Subscribes to the `Bus` and returns a `BroadcastStream` that receives events.
	///
	/// # Returns
	///
	/// A `BroadcastStream` that receives events emitted to the `Bus`.
	pub fn subscribe(&self) -> impl Stream<Item = T> {
		BroadcastStream::new(self.state.subscribe()).filter_map(Result::ok)
	}
}
