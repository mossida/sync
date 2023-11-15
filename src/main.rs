use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use warp::Filter;

mod automations;
mod entities;
mod events;
mod models;
mod types;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    DB.connect::<Ws>("localhost:8000").await?;
    DB.use_ns("general").use_db("main").await?;

    let routes = warp::any().and(entities::api::routes()).with(
        warp::cors()
            .allow_methods(["GET", "POST", "PATCH", "DELETE"])
            .allow_any_origin(),
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}
