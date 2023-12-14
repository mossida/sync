#![forbid(unsafe_code)]

extern crate core;

use figment::providers::Format;
use figment::{providers::Toml, Figment};
use once_cell::sync::Lazy;
use surreal_id::NewId;
use warp::Filter;

use crate::api::handlers::handle_rejection;
use crate::config::Config;
use crate::errors::Error;

mod api;
mod automations;
mod config;
mod db;
mod devices;
mod entities;
mod errors;
mod events;
mod helpers;
mod integrations;
mod models;
mod scheduler;
mod secrets;
mod ws;

static CONFIG: Lazy<Config> = Lazy::new(|| {
    let figment: Figment = Figment::new().merge(Toml::file("config.toml"));

    figment.extract().unwrap()
});

#[tokio::main(flavor = "multi_thread")]
async fn main() -> miette::Result<(), Error> {
    secrets::init();
    env_logger::init();

    db::init().await?;
    scheduler::init().await?;

    let routes = warp::any()
        .and(
            entities::endpoints::routes()
                .or(events::endpoints::routes())
                .or(ws::endpoints::route()),
        )
        .with(
            warp::cors()
                .allow_methods(["GET", "POST", "PATCH", "DELETE"])
                .allow_any_origin(),
        )
        .with(warp::log(&CONFIG.general.log_level))
        .recover(handle_rejection);

    // FIXME: Use better error handling of rejections
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
