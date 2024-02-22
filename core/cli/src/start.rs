use clap::Args;

#[derive(Args, Debug)]
pub struct StartCommandArgs {}

#[tracing::instrument]
pub async fn init(_: StartCommandArgs) -> err::Result<()> {
	dbm::init().await?;
	let _ = net::init().await;

	Ok(())
}
