use resources::database;
use resources::secrets;

use utils::error::log;

#[tokio::main]
async fn main() {
    secrets::init();
    env_logger::init();

    log(database::init().await);
}
