use std::ops::Deref;

use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Database;
use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;

use crate::CONFIG;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn init() -> surrealdb::Result<()> {
    DB.connect::<Ws>(&CONFIG.database.host).await?;
    DB.signin(Database {
        namespace: "general",
        database: "main",
        username: &CONFIG.database.username,
        password: &CONFIG.database.password,
    })
    .await?;

    // Run migrations
    MigrationRunner::new(&DB)
        .up()
        .await
        .expect("Failed to apply migrations");

    Ok(())
}

pub fn get() -> &'static Surreal<Client> {
    DB.deref()
}
