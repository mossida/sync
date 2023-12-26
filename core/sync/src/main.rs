#![forbid(unsafe_code)]

use integrations::scheduler;
use resources::{database, secrets};
use utils::error::log;

#[tokio::main]
async fn main() {
    secrets::init();
    env_logger::init();

    log(database::init().await);
    log(scheduler::init().await);
}
