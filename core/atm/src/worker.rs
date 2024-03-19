use std::sync::Arc;

use bus::Event;
use dashmap::DashSet;
use tracing::trace;
use trg::Trigger;

use ractor::{
	async_trait,
	factory::{
		FactoryMessage, WorkerBuilder as Builder, WorkerId, WorkerMessage, WorkerStartContext,
	},
	Actor, ActorProcessingErr, ActorRef,
};

pub type WorkerKey = u64;

#[derive(Clone)]
pub struct Worker {
	pub triggers: Arc<DashSet<Trigger>>,
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
	type Msg = WorkerMessage<WorkerKey, Event>;
	type State = WorkerState<Self::Arguments>;
	type Arguments = WorkerStartContext<WorkerKey, Event>;

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
			WorkerMessage::Dispatch(event) => {
				trace!("Received event to process in engine: ${:?}", event.msg);

				let _: Vec<Trigger> = self
					.triggers
					.iter()
					.filter_map(|trigger| {
						trigger.trigger(event.msg.clone()).ok().map(|_| trigger.to_owned())
					})
					.collect();

				state
					.context
					.factory
					.send_message(FactoryMessage::Finished(state.context.wid, event.key))?;
			}
		};

		Ok(())
	}
}
