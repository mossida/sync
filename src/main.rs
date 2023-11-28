use figment::{Figment, providers::Toml};
use figment::providers::Format;
use once_cell::sync::Lazy;
use warp::Filter;

use crate::api::handlers::handle_rejection;
use crate::config::Config;

mod api;
mod automations;
mod config;
mod db;
mod devices;
mod entities;
mod events;
mod helpers;
mod integrations;
mod models;
mod scheduler;
mod types;
mod ws;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let figment: Figment = Figment::new().merge(Toml::file("config.toml"));

    figment.extract().unwrap()
});

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    env_logger::init();

    let _ = db::init().await;
    let mut scheduler_guard = scheduler::get().await;

    scheduler_guard.execute();

    let routes = warp::any()
        .and(
            entities::endpoints::routes()
                .or(events::enpoints::routes())
                .or(ws::enpoints::route()),
        )
        .with(
            warp::cors()
                .allow_methods(["GET", "POST", "PATCH", "DELETE"])
                .allow_any_origin(),
        )
        .with(warp::log(&CONFIG.general.log_level))
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}
