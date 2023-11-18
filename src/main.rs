use std::collections::HashMap;

use figment::providers::Format;
use figment::{providers::Toml, Figment};
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use tokio::sync::{Mutex, MutexGuard};
use warp::Filter;

use crate::config::Config;
use crate::integrations::adapter::AdapterManager;
use crate::scheduler::Scheduler;

mod automations;
mod config;
mod entities;
mod events;
mod helpers;
mod integrations;
mod models;
mod scheduler;
mod types;

/*
   https://github.com/rust-unofficial/awesome-rust
   Useful libraries that needs implementation:

   - Rayon (automatic parallelism)
   - Anyhow (error handling)
   - Rkyv (faster deserialization)
   - PyO3 (to interact with python modules)
   - Hyper (HTTP handling)
*/

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    let figment: Figment = Figment::new().merge(Toml::file("config.toml"));

    Mutex::new(figment.extract().unwrap())
});

static SCHEDULER: Lazy<Mutex<Scheduler>> = Lazy::new(|| {
    Mutex::new(Scheduler {
        runners: HashMap::new(),
        adapters: HashMap::new(),
    })
});

#[tokio::main(flavor = "multi_thread")]
async fn main() -> surrealdb::Result<()> {
    let config: MutexGuard<Config> = CONFIG.lock().await;
    let mut scheduler: MutexGuard<Scheduler> = SCHEDULER.lock().await;

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
