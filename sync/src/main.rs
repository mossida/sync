#![forbid(unsafe_code)]

#[tokio::main]
async fn main() -> miette::Result<()> {
	tracing_subscriber::fmt::init();

	cli::init().await
}
