#![forbid(unsafe_code)]

use warp::Filter;

use integrations::scheduler;
use models::component;
use models::component::Component;
use resources::{database, secrets};
use utils::error::log;

#[tokio::main]
async fn main() {
    secrets::init();
    env_logger::init();

    log(database::init().await);
    log(scheduler::init().await);

    let components: Vec<Component> = database::get().select(component::RESOURCE).await.unwrap();
    scheduler::register(components).await;

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
