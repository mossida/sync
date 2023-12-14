use std::ops::Deref;

use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Database;
use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;

use crate::errors::Error;
use crate::CONFIG;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn init() -> miette::Result<(), Error> {
    DB.connect::<Ws>(&CONFIG.database.host).await?;
    DB.signin(Database {
        namespace: "general",
        database: "main",
        username: &CONFIG.database.username,
        password: &CONFIG.database.password,
    })
    .await?;

    // Run migrations
    MigrationRunner::new(&DB).up().await.unwrap();

    Ok(())
}

pub fn get() -> &'static Surreal<Client> {
    DB.deref()
}
