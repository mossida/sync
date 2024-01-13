use ractor::concurrency::Duration;
use ractor::ActorRef;

pub(crate) type WorkerKey = String;

pub enum SchedulerMessage {
	Ping(String),
	PollInterface(Duration, ActorRef<InterfaceMessage>),
}

pub enum AdapterMessage {
	Ping(String),
	SpawnInterfaces,
	Update,
}

pub enum InterfaceMessage {
	Ping(String),
	Update,
}
