use dbm::{
	fetch::Fetch,
	resource::{Base, Resource},
};
use err::Error;
use r#type::ServiceType;
use ractor::{registry, ActorRef, Message, RpcReplyPort};
use serde::{Deserialize, Serialize};
use tracing::error;

pub mod r#type;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
	id: dbm::Id,
	#[serde(flatten)]
	call: ServiceCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCall {
	pub payload: serde_json::Value,
	pub service_type: Fetch<ServiceType>,
}

impl Base for Service {
	const RESOURCE: &'static str = "service";
}

impl Resource for Service {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

impl Service {
	pub fn is(&self, service_type: &ServiceType) -> bool {
		self.call.service_type.id().clone() == service_type.id
	}

	pub async fn call<F, M, R>(self, builder: F) -> Result<(), Error>
	where
		F: FnOnce(RpcReplyPort<R>, Service) -> M,
		M: Message,
	{
		let service_type = self.call.service_type.fetch().await?;

		let entry: Option<ActorRef<M>> =
			registry::where_is(service_type.id().to_raw()).map(Into::into);

		if let Some(actor) = entry {
			// TODO: Use result to handle errors
			let _ = actor.call(|port| builder(port, self), None).await;
		} else {
			error!("No actor found for service {:?}, this must not happen!", self.id());
		}

		Ok(())
	}
}
