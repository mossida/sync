#![forbid(unsafe_code)]

use integrations::dispatcher;
use models::component;
use models::component::Component;
use resources::{database, secrets};
use utils::error::log;

mod net;
mod rpc;

// https://github.com/surrealdb/surrealdb/blob/7b197c2acdcfdf9161813e9f904b8f2bc40db3f9/src/net/mod.rs

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	secrets::init();

	log(database::init().await);
	log(dispatcher::init().await);

	let components: Vec<Component> = database::get().select(component::RESOURCE).await.unwrap();
	log(dispatcher::register(components).await);

	log(net::init().await);
}
