use clap::Args;

use vnd::spawner;

#[derive(Args, Debug)]
pub struct StartCommandArgs {}

#[tracing::instrument]
pub async fn init(_: StartCommandArgs) -> err::Result<()> {
	//let token = CancellationToken::new();

	// Critical components
	bus::init();
	mqtt::init();

	dbm::init().await?;

	// Non-critical components
	// TODO: capture errors as logs instead of panicking
	spawner::init().await?;

	// Interface components
	mqtt::serve().await;
	net::serve().await?;

	Ok(())
}
