use clap::Args;
use components::spawner;

#[derive(Args, Debug)]
pub struct StartCommandArgs {}

#[tracing::instrument]
pub async fn init(_: StartCommandArgs) -> err::Result<()> {
	// Critical components
	bus::init();
	mqtt::init();

	dbm::init().await?;

	// Non-critical components
	atm::init().await?;
	spawner::init().await?;

	// Interface components
	mqtt::serve();
	net::serve().await?;

	Ok(())
}
