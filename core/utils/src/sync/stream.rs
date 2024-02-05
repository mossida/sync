use futures::{future, Stream, StreamExt};
use ractor::async_trait;
use tokio::{
	sync::broadcast::{self, Receiver},
	task::JoinHandle,
};

#[async_trait]
trait BroadcastStream<T> {
	async fn broadcast(self, buffer: usize) -> (Receiver<T>, JoinHandle<()>)
	where
		T: Clone + Send + 'static;
}

#[async_trait]
impl<T, K: Stream<Item = T> + StreamExt> BroadcastStream<T> for K
where
	K: Send + Unpin + 'static,
	T: Clone + Send + 'static,
{
	async fn broadcast(self, buffer: usize) -> (Receiver<T>, JoinHandle<()>) {
		let (sender, receiver) = broadcast::channel::<T>(buffer);

		let handle = tokio::task::spawn(self.for_each(move |item| {
			let _ = sender.send(item);
			future::ready(())
		}));

		(receiver, handle)
	}
}
