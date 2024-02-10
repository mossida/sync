use bytes::Bytes;

use models::event::Event;
use ractor::concurrency::Duration;
use ractor::ActorRef;

pub(crate) type WorkerKey = String;

pub enum ServiceRequest {
	PollInterface(Duration, ActorRef<InterfaceMessage>),
	RealtimeInterface(ActorRef<InterfaceMessage>),
}

pub enum BusType {
	Event(Event),
}

pub enum Dispatch {
	Ping(String),
	Bus(BusType),
	ServiceRequest(ServiceRequest),
}

pub enum AdapterMessage {
	Ping(String),
	SpawnInterfaces,
	Update,
}

pub enum InterfaceMessage {
	Ping(String),
	Update,
	UpdateWithData(Bytes),
}
