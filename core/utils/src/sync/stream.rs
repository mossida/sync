use async_trait::async_trait;
use futures::{future, Stream, StreamExt};
use tokio::{
	sync::broadcast::{self, Receiver},
	task::JoinHandle,
};

#[async_trait]
pub trait BroadcastStream: Stream {
	async fn broadcast(self, buffer: usize) -> (Receiver<Self::Item>, JoinHandle<()>)
	where
		Self::Item: Clone + Send + 'static;
}

#[async_trait]
impl<K> BroadcastStream for K
where
	K: Stream + Send + Unpin + 'static,
	K::Item: Clone + Send + 'static,
{
	async fn broadcast(self, buffer: usize) -> (Receiver<Self::Item>, JoinHandle<()>) {
		let (sender, receiver) = broadcast::channel(buffer);

		let handle = tokio::task::spawn(self.for_each(move |item| {
			let _ = sender.send(item);
			future::ready(())
		}));

		(receiver, handle)
	}
}
