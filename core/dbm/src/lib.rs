use err::{Error, MigrationError};
use once_cell::sync::Lazy;
use surrealdb::{engine::any::Any, opt::auth::Root, Surreal};
use surrealdb_migrations::MigrationRunner;
use tracing::{info, instrument, trace};

pub use surrealdb::sql::Id;

mod relation;
pub mod resource;

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

	MigrationRunner::new(&DB).up().await.map_err(|e| {
		Error::MigrationError(MigrationError {
			message: e.to_string(),
		})
	})?;

	info!("Database ready");

	Ok(())
}
