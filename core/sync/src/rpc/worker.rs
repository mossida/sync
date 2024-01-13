use axum::async_trait;
use axum::extract::ws::Message;
use ractor::factory::{
	FactoryMessage, WorkerBuilder as Builder, WorkerId, WorkerMessage, WorkerStartContext,
};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub type WorkerKey = Uuid;

#[derive(Clone)]
pub struct Worker {
	pub sender: Sender<Message>,
	pub canceller: CancellationToken,
}

pub struct WorkerBuilder {
	pub sender: Sender<Message>,
	pub canceller: CancellationToken,
}

impl Builder<Worker> for WorkerBuilder {
	fn build(&self, _: WorkerId) -> Worker {
		Worker {
			sender: self.sender.clone(),
			canceller: self.canceller.clone(),
		}
	}
}

pub struct WorkerState<T> {
	pub sender: Sender<Message>,
	pub context: T,
}

#[async_trait]
impl Actor for Worker {
	type Msg = WorkerMessage<WorkerKey, Message>;
	type State = WorkerState<Self::Arguments>;
	type Arguments = WorkerStartContext<WorkerKey, Message>;

	async fn pre_start(
		&self,
		_: ActorRef<Self::Msg>,
		args: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		Ok(WorkerState {
			sender: self.sender.clone(),
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
				state
					.context
					.factory
					.send_message(FactoryMessage::WorkerPong(state.context.wid, time.elapsed()))?;
			}
			WorkerMessage::Dispatch(job) => match job.msg {
				Message::Text(_) => {
					state
						.sender
						.send(Message::Text(
							format!("Hello from worker: {}", state.context.wid).into(),
						))
						.await?;

					state
						.context
						.factory
						.send_message(FactoryMessage::Finished(state.context.wid, job.key))?;
				}
				Message::Binary(_) => {}
				Message::Close(_) => {
					self.canceller.cancel();
				}
				_ => {}
			},
		}

		Ok(())
	}
}
