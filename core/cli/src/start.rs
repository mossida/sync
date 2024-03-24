use clap::Args;

use vnd::spawner;

#[derive(Args, Debug)]
pub struct StartCommandArgs {}

#[tracing::instrument]
pub async fn init(_: StartCommandArgs) -> err::Result<()> {
	// Critical components
	bus::init();
	dbm::init().await?;

	// Non-critical components
	atm::init().await?;
	spawner::init().await?;

	// Interface components
	mqtt::init();
	net::serve().await?;

	Ok(())
}
