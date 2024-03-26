use std::sync::Arc;

use ractor::{
	async_trait,
	factory::{FactoryMessage, Job, WorkerBuilder, WorkerMessage, WorkerStartContext},
	Actor, ActorProcessingErr, ActorRef,
};

use crate::{RefContext, Vendor};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Task {
	Poll,
	Consume,
}

#[derive(Clone)]
pub struct Worker<V>
where
	V: Vendor,
{
	pub vendor: Arc<V>,
	pub context: RefContext<V>,
}

pub struct Builder<V>
where
	V: Vendor,
{
	pub vendor: Arc<V>,
	pub context: RefContext<V>,
}

impl<V> WorkerBuilder<Worker<V>> for Builder<V>
where
	V: Vendor,
{
	fn build(&self, _: ractor::factory::WorkerId) -> Worker<V> {
		Worker {
			vendor: self.vendor.clone(),
			context: self.context.clone(),
		}
	}
}

#[async_trait]
impl<V> Actor for Worker<V>
where
	V: Vendor,
{
	type Msg = WorkerMessage<Task, Option<V::PollData>>;
	type Arguments = WorkerStartContext<Task, Option<V::PollData>>;
	type State = Self::Arguments;

	async fn pre_start(
		&self,
		_: ActorRef<Self::Msg>,
		context: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		Ok(context)
	}

	async fn handle(
		&self,
		_: ActorRef<Self::Msg>,
		message: Self::Msg,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		match message {
			WorkerMessage::FactoryPing(time) => {
				state
					.factory
					.send_message(FactoryMessage::WorkerPong(state.wid, time.elapsed()))?;
			}
			WorkerMessage::Dispatch(job) => {
				let Job {
					key: task,
					msg: data,
					..
				} = job;

				match task {
					Task::Poll => {
						let result = self.vendor.poll(self.context.clone()).await;

						if let Ok(data) = result {
							if data.is_some() {
								state.factory.send_message(FactoryMessage::Dispatch(Job {
									key: Task::Consume,
									msg: data,
									options: Default::default(),
								}))?;
							}
						} else if V::STOP_ON_ERROR {
							let _ = state
								.factory
								.stop_and_wait(None, Some(V::POLLING_INTERVAL))
								.await
								.inspect_err(|_| state.factory.kill());
						}
					}
					Task::Consume => {
						if let Some(data) = data {
							self.vendor.consume(self.context.clone(), data).await?;
						}
					}
				};

				state.factory.send_message(FactoryMessage::Finished(state.wid, task))?;
			}
		};

		Ok(())
	}
}
