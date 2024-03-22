use crate::Vendor;
use bus::Event;
use ractor::RpcReplyPort;
use svc::Service;

pub mod actor;
pub mod context;

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
	PollingInstant,
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
}
