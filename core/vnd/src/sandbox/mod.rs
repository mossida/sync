use std::marker::PhantomData;

use crate::Vendor;
use bus::Event;
use ractor::{ActorProcessingErr, RpcReplyPort};
use svc::Service;

pub mod actor;
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
	vendor: PhantomData<V>,
}

impl<V> Sandbox<V>
where
	V: Vendor,
{
	pub fn new() -> Self {
		Self {
			vendor: PhantomData,
		}
	}
}
