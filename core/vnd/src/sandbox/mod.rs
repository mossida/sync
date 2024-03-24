use crate::Vendor;
use bus::Event;
use ractor::{Actor, ActorCell, ActorProcessingErr, ActorRef, RpcReplyPort};
use svc::Service;

use self::worker::Worker;

pub mod actor;
pub mod context;
pub mod worker;

pub type SandboxError = ActorProcessingErr;

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

pub enum SandboxMessage {
	Event(Event),
	Request(Request, RpcReplyPort<Response>),
}

impl From<Event> for SandboxMessage {
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

		if V::POLLING_INTERVAL.is_zero() {
			let _ = worker.send_message(());
		} else {
			worker.send_interval(V::POLLING_INTERVAL, || ());
		}

		Ok(worker)
	}
}
