use clap::Args;
use tokio_util::sync::CancellationToken;

#[derive(Args, Debug)]
pub struct StartCommandArgs {}

#[tracing::instrument]
pub async fn init(_: StartCommandArgs) -> err::Result<()> {
	let token = CancellationToken::new();

	dbm::init().await?;

	let _ = mqtt::serve(token.child_token()).await;
	let _ = net::init().await;

	vnd::spawner::init().await?;

	Ok(())
}
