use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

mod events;

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    DB.connect::<Ws>("ws://127.0.0.0:8000").await?;

    Ok(())
}
