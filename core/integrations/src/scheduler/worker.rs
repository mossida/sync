use std::sync::Arc;

use dashmap::DashMap;
use ractor::concurrency::JoinHandle;
use ractor::factory::{
    FactoryMessage, WorkerBuilder as Builder, WorkerId, WorkerMessage, WorkerStartContext,
};
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};

use crate::scheduler::models::{InterfaceMessage, SchedulerMessage, WorkerKey};

#[derive(Default)]
pub(super) struct Worker {
    #[allow(dead_code)]
    pub handlers: Arc<DashMap<String, JoinHandle<()>>>,
}

#[derive(Default)]
pub(super) struct WorkerBuilder {
    pub handlers: Arc<DashMap<String, JoinHandle<()>>>,
}

impl Builder<Worker> for WorkerBuilder {
    fn build(&self, _: WorkerId) -> Worker {
        Worker {
            handlers: self.handlers.clone(),
        }
    }
}

#[async_trait]
impl Actor for Worker {
    type Msg = WorkerMessage<WorkerKey, SchedulerMessage>;
    type State = Self::Arguments;
    type Arguments = WorkerStartContext<WorkerKey, SchedulerMessage>;

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
                    .cast(FactoryMessage::WorkerPong(state.wid, time.elapsed()))?;
            }
            WorkerMessage::Dispatch(job) => match job.msg {
                SchedulerMessage::Ping(message) => {
                    dbg!(message);
                }
                SchedulerMessage::PollInterface(interval, interface) => {
                    self.handlers
                        .entry(interface.get_name().unwrap())
                        .insert(interface.send_interval(interval, || InterfaceMessage::Update));

                    state
                        .factory
                        .cast(FactoryMessage::Finished(state.wid, job.key))?;
                }
            },
        }

        Ok(())
    }
}
