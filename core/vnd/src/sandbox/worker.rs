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
	type Arguments = V::Context;
	type State = V::Context;

	async fn pre_start(
		&self,
		_: ActorRef<Self::Msg>,
		context: Self::Arguments,
	) -> Result<Self::State, ActorProcessingErr> {
		Ok(context)
	}

	async fn handle(
		&self,
		_: ActorRef<Self::Msg>,
		_: Self::Msg,
		context: &mut Self::State,
	) -> Result<(), ActorProcessingErr> {
		let data = self.vendor.poll(context).await?;
		let _ = self.vendor.process(context, data).await?;

		Ok(())
	}
}
