use std::ops::Deref;

use crate::configuration;
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Database;
use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;
use utils::error::Error;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn init() -> utils::types::Result<()> {
    let config = configuration::get();
    DB.connect::<Ws>(&config.database.host).await?;
    DB.signin(Database {
        namespace: &config.database.namespace,
        database: &config.database.database,
        username: &config.database.username,
        password: &config.database.password,
    })
    .await?;

    // Run migrations
    MigrationRunner::new(&DB)
        .up()
        .await
        .map_err(|e| Error::Migration(e.to_string()))?;

    Ok(())
}

pub fn get() -> &'static Surreal<Client> {
    DB.deref()
}
