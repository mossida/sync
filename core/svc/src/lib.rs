use dbm::resource::{Base, Resource};
use ractor::{ActorRef, Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
	id: dbm::Id,
}

impl Service {
	pub fn execute<T>(&self, cell: ActorRef<T>)
	where
		T: From<Service> + Message,
	{
		let _ = cell.send_message(self.clone().into());
	}
}

impl Base for Service {
	const RESOURCE: &'static str = "service";
}

impl Resource for Service {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}
