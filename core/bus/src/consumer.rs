use futures::{Future, Stream};
use ractor::{
	factory::{FactoryMessage, Job, JobKey},
	ActorRef,
};

use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::error;

pub trait Consumer: Stream {
	fn consume<C, F>(self, closure: C) -> JoinHandle<()>
	where
		C: Fn(Self::Item) -> F + Send + 'static,
		F: Future<Output = ()> + Send + 'static,
		Self: Sized + Send + 'static,
		Self::Item: Send + 'static,
	{
		let mut stream = Box::pin(self);

		tokio::spawn(async move {
			while let Some(event) = stream.next().await {
				closure(event).await;
			}
		})
	}

	fn to_actor<M>(self, cell: ActorRef<M>) -> CancellationToken
	where
		M: From<Self::Item> + Send + 'static,
		Self: Sized + Send + 'static,
	{
		let token = CancellationToken::new();
		let inner_token = token.child_token();
		let mut stream = Box::pin(self);

		tokio::spawn(async move {
			loop {
				tokio::select! {
					_ = inner_token.cancelled() => break,
					Some(event) = stream.next() => {
						let result = cell.send_message(event.into());

						if result.is_err() {
							error!("Failed to send message to actor");
							inner_token.cancel();
						}
					}
				}
			}
		});

		token
	}

	fn to_factory<Key, Message, Builder>(
		self,
		cell: ActorRef<FactoryMessage<Key, Message>>,
		builder: Builder,
	) -> CancellationToken
	where
		Key: JobKey,
		Message: From<Self::Item> + Send + 'static,
		Builder: Fn(&Self::Item) -> Key + Send + 'static,
		Self: Sized + Send + 'static,
	{
		let token = CancellationToken::new();
		let inner_token = token.child_token();
		let mut stream = Box::pin(self);

		tokio::spawn(async move {
			loop {
				tokio::select! {
					_ = inner_token.cancelled() => break,
					Some(event) = stream.next() => {
						let result = cell.send_message(FactoryMessage::Dispatch(Job {
							key: builder(&event),
							msg: event.into(),
							options: Default::default(),
						}));

						if result.is_err() {
							error!("Failed to send message to actor");
							inner_token.cancel();
						}
					}
				}
			}
		});

		token
	}
}

impl<St: ?Sized> Consumer for St where St: Stream {}
