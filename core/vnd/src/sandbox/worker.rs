use ractor::{async_trait, Actor, ActorProcessingErr, ActorRef};

use crate::Vendor;

#[derive(Clone)]
pub struct Worker<V> {
	pub vendor: V,
}

#[async_trait]
impl<V> Actor for Worker<V>
where
	V: Vendor,
{
	type Msg = ();
	type Arguments = ();
	type State = ();

	async fn pre_start(
		&self,
		_: ActorRef<Self::Msg>,
		_: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		Ok(())
	}

	async fn handle(
		&self,
		_: ActorRef<Self::Msg>,
		_: Self::Msg,
		_: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		self.vendor.run().await;
		Ok(())
	}
}
