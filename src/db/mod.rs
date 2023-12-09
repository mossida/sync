use std::ops::Deref;

use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Database;
use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;

use crate::CONFIG;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn init() {
    DB.connect::<Ws>(&CONFIG.database.host)
        .await
        .expect("Failed to connect to database");
    DB.signin(Database {
        namespace: "general",
        database: "main",
        username: &CONFIG.database.username,
        password: &CONFIG.database.password,
    })
    .await
    .expect("Failed to sign in to database");

    // Run migrations
    MigrationRunner::new(&DB)
        .up()
        .await
        .expect("Failed to apply migrations");
}

pub fn get() -> &'static Surreal<Client> {
    DB.deref()
}
