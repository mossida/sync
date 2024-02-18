use std::ops::Deref;

use cnf::CONFIG;
use once_cell::sync::Lazy;
use surrealdb::{engine::any::Any, opt::auth::Root};

use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;

use utils::error::Error;

static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);

pub async fn init() -> utils::types::Result<()> {
	let config = &CONFIG.database;

	DB.connect(&config.endpoint).await?;
	DB.use_ns(&config.namespace).await?;
	DB.use_db(&config.database).await?;

	if let (Some(username), Some(password)) = (&config.username, &config.password) {
		DB.signin(Root {
			username,
			password,
		})
		.await?;
	}

	// Run migrations
	MigrationRunner::new(&DB).up().await.map_err(|e| Error::Migration(e.to_string()))?;

	Ok(())
}

pub fn get() -> &'static Surreal<Any> {
	DB.deref()
}
