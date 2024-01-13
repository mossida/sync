use std::sync::OnceLock;

use ractor::factory::{Factory, FactoryMessage, Job, RoutingMode};
use ractor::{Actor, ActorRef};

use crate::scheduler::models::{SchedulerMessage, WorkerKey};
use crate::scheduler::spawner::Spawner;
use crate::scheduler::worker::WorkerBuilder;

static FACTORY: OnceLock<ActorRef<FactoryMessage<WorkerKey, SchedulerMessage>>> = OnceLock::new();
static SPAWNER: OnceLock<ActorRef<()>> = OnceLock::new();

pub(super) async fn init() -> utils::types::Result<()> {
	let builder: WorkerBuilder = Default::default();
	let (factory, _) = Actor::spawn(
		Some(String::from("scheduler")),
		Factory {
			worker_count: 10,
			worker_parallel_capacity: 2,
			routing_mode: RoutingMode::<WorkerKey>::Queuer,
			..Default::default()
		},
		Box::new(builder),
	)
	.await
	.unwrap();

	let (spawner, _) = Actor::spawn(Some(String::from("spawner")), Spawner {}, ()).await.unwrap();

	SPAWNER.set(spawner).unwrap();
	FACTORY.set(factory).unwrap();
	Ok(())
}

pub(super) async fn send(message: SchedulerMessage) -> utils::types::Result<()> {
	let factory = FACTORY.get().ok_or("Factory not initialized").unwrap();
	factory
		.cast(FactoryMessage::Dispatch(Job {
			key: WorkerKey::from("worker"),
			msg: message,
			options: Default::default(),
		}))
		.unwrap();
	Ok(())
}

pub(super) fn get() -> ActorRef<FactoryMessage<WorkerKey, SchedulerMessage>> {
	FACTORY.get().ok_or("Factory not initialized").unwrap().clone()
}

pub(super) fn get_spawner() -> ActorRef<()> {
	SPAWNER.get().ok_or("Spawner not initialized").unwrap().clone()
}
