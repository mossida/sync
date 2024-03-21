use clap::Args;

use vnd::spawner;

#[derive(Args, Debug)]
pub struct StartCommandArgs {}

#[tracing::instrument]
pub async fn init(_: StartCommandArgs) -> err::Result<()> {
	//let token = CancellationToken::new();

	// Critical components
	bus::init();
	dbm::init().await?;

	// Non-critical components
	// TODO: capture errors as logs instead of panicking
	atm::init().await?;
	spawner::init().await?;

	// TODO: refactor serve method
	//let _ = mqtt::serve(token.child_token()).await;

	// Interface components
	net::serve().await?;

	Ok(())
}
