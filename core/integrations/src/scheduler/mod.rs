mod factory;
pub mod models;
mod worker;

pub async fn init() -> utils::types::Result<()> {
    factory::init().await
}

pub async fn send(message: models::SchedulerMessage) -> utils::types::Result<()> {
    factory::send(message).await
}
