#![forbid(unsafe_code)]

use dbm::DB;
use vnd::Component;

#[tokio::main]
async fn main() -> err::Result<()> {
	tracing_subscriber::fmt::init();

	cli::init().await
}
