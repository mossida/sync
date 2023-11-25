use std::ops::Deref;

use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

use crate::CONFIG;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn init() -> surrealdb::Result<()> {
    DB.connect::<Ws>(&CONFIG.database.host).await?;
    DB.use_ns("general").use_db("main").await?;

    Ok(())
}

pub fn get() -> &'static Surreal<Client> {
    DB.deref()
}
