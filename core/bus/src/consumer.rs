use futures::Stream;
use ractor::ActorRef;

use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::error;

pub trait Consumer: Stream {
	fn to_actor<M>(mut self, cell: ActorRef<M>) -> CancellationToken
	where
		M: From<Self::Item> + Send + Sync + 'static,
		Self: Unpin + Sized + Send + Sync + 'static,
	{
		let token = CancellationToken::new();
		let inner_token = token.child_token();

		tokio::spawn(async move {
			loop {
				tokio::select! {
					_ = inner_token.cancelled() => break,
					Some(event) = self.next() => {
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
}

impl<St: ?Sized> Consumer for St where St: Stream {}
