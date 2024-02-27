use std::sync::Arc;

use axum::async_trait;
use axum::extract::ws::Message;

use ractor::factory::{
	FactoryMessage, WorkerBuilder as Builder, WorkerId, WorkerMessage, WorkerStartContext,
};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use tokio::sync::mpsc::Sender;
use tracing::trace;

use crate::request::Request;
use crate::response::IntoResponse;

pub type WorkerKey = u64;

#[derive(Clone)]
pub struct Worker {
	pub sender: Arc<Sender<Message>>,
}

impl Builder<Worker> for Worker {
	fn build(&self, _: WorkerId) -> Worker {
		self.clone()
	}
}

pub struct WorkerState<T> {
	pub context: T,
}

#[async_trait]
impl Actor for Worker {
	type Msg = WorkerMessage<WorkerKey, Request>;
	type State = WorkerState<Self::Arguments>;
	type Arguments = WorkerStartContext<WorkerKey, Request>;

	async fn pre_start(
		&self,
		_: ActorRef<Self::Msg>,
		args: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		Ok(WorkerState {
			context: args,
		})
	}

	async fn handle(
		&self,
		_: ActorRef<Self::Msg>,
		message: Self::Msg,
		state: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		match message {
			WorkerMessage::FactoryPing(time) => {
				// Pong back to factory
				state
					.context
					.factory
					.send_message(FactoryMessage::WorkerPong(state.context.wid, time.elapsed()))?;
			}
			WorkerMessage::Dispatch(job) => {
				trace!("Dispatching job {:?} with actor {:?}", job.key, state.context.wid);

				// Process request
				let data = job.msg.await;
				let response = data.into_response(job.key);

				// Send back to client
				self.sender.send(response.try_into()?).await?;

				// Notify factory the job is done so this worker is free again
				state
					.context
					.factory
					.send_message(FactoryMessage::Finished(state.context.wid, job.key))?;
			}
		}

		Ok(())
	}
}
