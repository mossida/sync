use std::ops::Deref;

use once_cell::sync::Lazy;
use surrealdb::engine::local::{Db, RocksDb};

use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;

use utils::error::Error;

use crate::configuration;

static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

pub async fn init() -> utils::types::Result<()> {
	let config = configuration::get();
	DB.connect::<RocksDb>(&config.database.storage).await?;

	// Run migrations
	MigrationRunner::new(&DB).up().await.map_err(|e| Error::Migration(e.to_string()))?;

	Ok(())
}

pub fn get() -> &'static Surreal<Db> {
	DB.deref()
}
