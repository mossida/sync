use std::sync::Mutex;

use figment::providers::Format;
use figment::{providers::Toml, Figment};
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use warp::Filter;

use crate::config::Config;

mod automations;
mod config;
mod entities;
mod events;
mod models;
mod types;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let figment: Figment = Figment::new().merge(Toml::file("config.toml"));

    Mutex::new(figment.extract().unwrap())
});

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let config = CONFIG.lock().unwrap();

    DB.connect::<Ws>(&config.database.host).await?;
    DB.use_ns("general").use_db("main").await?;

    let routes = warp::any().and(entities::api::routes()).with(
        warp::cors()
            .allow_methods(["GET", "POST", "PATCH", "DELETE"])
            .allow_any_origin(),
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}
