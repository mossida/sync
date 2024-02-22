#![forbid(unsafe_code)]

#[tokio::main]
async fn main() -> err::Result<()> {
	tracing_subscriber::fmt::init();

	cli::init().await
}
