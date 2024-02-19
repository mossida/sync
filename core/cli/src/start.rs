use clap::Args;
use err::Error;

#[derive(Args, Debug)]
pub struct StartCommandArgs {}

#[tracing::instrument]
pub async fn init(_: StartCommandArgs) -> Result<(), Error> {
	dbm::init().await?;
	let _ = net::init().await;

	Ok(())
}
