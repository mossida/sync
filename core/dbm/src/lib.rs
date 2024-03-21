use std::pin::Pin;

use err::Error;
use futures::Future;
use once_cell::sync::Lazy;
use surrealdb::{engine::any::Any, opt::auth::Root, Surreal};
use tracing::{info, instrument, trace};

pub use id::Id;
pub use id::IdKind;

mod id;

pub mod link;
pub mod relation;
pub mod resource;

/// Internal type to indicate an async method
type IntoFuture<'r, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'r>>;

pub static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

#[instrument]
pub async fn init() -> Result<(), Error> {
	let config = &cnf::get().database;

	trace!("Connecting to the endpoint");
	DB.connect(&config.endpoint).await?;

	trace!("Using the namespace {}", &config.namespace);
	DB.use_ns(&config.namespace).await?;
	DB.use_db(&config.database).await?;

	if let (Some(username), Some(password)) = (&config.username, &config.password) {
		trace!("Credentials provided, signing in");
		DB.signin(Root {
			username,
			password,
		})
		.await?;
	}

	// TODO: Understand why runner fails parse
	/*MigrationRunner::new(&DB).up().await.map_err(|e| {
		Error::MigrationError(MigrationError {
			message: e.to_string(),
		})
	})?;*/

	info!("Database ready");

	Ok(())
}
