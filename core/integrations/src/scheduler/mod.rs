use hashbrown::hash_map::DefaultHashBuilder;
use priority_queue::PriorityQueue;
use rayon::prelude::*;

use ::models::component::Component;

use crate::Vendor;

mod factory;
pub mod models;
mod spawner;
mod worker;

pub async fn init() -> utils::types::Result<()> {
    factory::init().await
}

pub async fn send(message: models::SchedulerMessage) -> utils::types::Result<()> {
    factory::send(message).await
}

pub async fn register(components: Vec<Component>) -> utils::types::Result<()> {
    // FIXME: Define how to control vendors actors group
    // Create a priority queue from the components
    let mut queue: PriorityQueue<Component, u8, DefaultHashBuilder> = components
        .into_par_iter()
        .map(|component| {
            let priority = component.priority.clone();
            (component, priority)
        })
        .collect::<Vec<(Component, u8)>>()
        .into();

    // FIXME: Define how to control vendors actors group and handle errors
    while let Some((component, _)) = queue.pop() {
        let vendor: Vendor = serde_json::from_value(component.vendor.clone())?;
        vendor
            .build(component, factory::get_spawner().get_cell())
            .await?;
    }

    Ok(())
}
