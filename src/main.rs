use figment::providers::Format;
use figment::{providers::Toml, Figment};
use once_cell::sync::Lazy;
use warp::Filter;

use crate::config::Config;

mod automations;
mod config;
mod db;
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

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let figment: Figment = Figment::new().merge(Toml::file("config.toml"));

    figment.extract().unwrap()
});

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let _ = db::init().await;

    let routes = warp::any().and(entities::api::routes()).with(
        warp::cors()
            .allow_methods(["GET", "POST", "PATCH", "DELETE"])
            .allow_any_origin(),
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
