use bus::Consumer;
use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};
use tracing::info;

use crate::Vendor;

pub struct Sandbox<V>
where
	V: Vendor,
{
	#[allow(dead_code)]
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

#[async_trait]
impl<C> Actor for Sandbox<C>
where
	C: Vendor,
{
	type Msg = C::Message;
	type Arguments = C::Configuration;
	type State = ();

	async fn pre_start(
		&self,
		myself: ActorRef<Self::Msg>,
		_: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		if C::SUBSCRIBE_BUS {
			let bus = bus::get();
			let _ = bus.subscribe().to_actor(myself);
		}

		info!("Component actor started");

		Ok(())
	}
}
