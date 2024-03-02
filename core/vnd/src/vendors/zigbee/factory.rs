use mqtt::rumqttd::protocol::Publish;
use ractor::{
	async_trait,
	factory::{FactoryMessage, WorkerBuilder as Builder, WorkerMessage, WorkerStartContext},
	Actor, ActorProcessingErr, ActorRef,
};

#[derive(Clone)]
pub struct Worker {}

impl Builder<Worker> for Worker {
	fn build(&self, _: usize) -> Worker {
		self.clone()
	}
}

pub struct WorkerState<T> {
	pub context: T,
}

#[async_trait]
impl Actor for Worker {
	type Msg = WorkerMessage<u64, Publish>;
	type State = WorkerState<Self::Arguments>;
	type Arguments = WorkerStartContext<u64, Publish>;

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
				state
					.context
					.factory
					.send_message(FactoryMessage::WorkerPong(state.context.wid, time.elapsed()))?;
			}
			WorkerMessage::Dispatch(job) => {
				let json = String::from_utf8(job.msg.payload.to_vec())?;
				println!("Received message: {}", json);

				// Deserialize the message
				// Parse it and load the state

				state
					.context
					.factory
					.send_message(FactoryMessage::Finished(state.context.wid, job.key))?;
			}
		};

		Ok(())
	}
}
