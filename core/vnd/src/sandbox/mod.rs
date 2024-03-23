use std::time::Duration;

use crate::Vendor;
use bus::Event;
use ractor::{Actor, ActorCell, ActorProcessingErr, ActorRef, RpcReplyPort};
use svc::Service;

use self::worker::Worker;

pub mod actor;
pub mod context;
pub mod worker;

pub enum Request {
	Call(Service),
}

pub enum Response {
	Handled,
	NotHandled,
}

impl<D, E> From<Result<D, E>> for Response {
	fn from(result: Result<D, E>) -> Self {
		match result {
			Ok(_) => Self::Handled,
			Err(_) => Self::NotHandled,
		}
	}
}

pub enum SandboxMessage<Message> {
	Event(Event),
	VendorMessage(Message),
	Request(Request, RpcReplyPort<Response>),
}

impl<Message> From<Event> for SandboxMessage<Message> {
	fn from(event: Event) -> Self {
		Self::Event(event)
	}
}

pub struct Sandbox<V>
where
	V: Vendor,
{
	vendor: V,
}

impl<V> Sandbox<V>
where
	V: Vendor,
{
	pub fn new(vendor: V) -> Self {
		Self {
			vendor,
		}
	}

	pub async fn spawn_worker(&self, cell: ActorCell) -> Result<ActorRef<()>, ActorProcessingErr> {
		let (worker, _) = Actor::spawn_linked(
			None,
			Worker {
				vendor: self.vendor.clone(),
			},
			(),
			cell,
		)
		.await?;

		if V::POLLING_INTERVAL > 0 {
			worker.send_interval(Duration::from_secs(V::POLLING_INTERVAL), || ());
		} else {
			// Run once
			let _ = worker.send_message(());
		}

		Ok(worker)
	}
}
