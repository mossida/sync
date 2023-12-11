use serde_json::Value;
use surreal_id::NewId;

use crate::api::rejections::Rejection;
use crate::db;
use crate::devices::models::DeviceId;
use crate::entities::models::{Entity, EntityId, Updates};

// FIXME: Return types are inconsistent
pub async fn create(
    entity: Entity,
    device_id: DeviceId,
    updates: Updates,
) -> Result<Vec<Entity>, Rejection> {
    create_multiple(vec![entity], device_id, updates).await
}

pub async fn create_multiple(
    entities: Vec<Entity>,
    device_id: DeviceId,
    updates: Updates,
) -> Result<Vec<Entity>, Rejection> {
    let mut insert_response = db::get()
        .query("INSERT INTO entity $entities")
        .bind(("entities", entities))
        .await?;

    // Only relate inserted entities
    let inserted_entities = insert_response.take::<Vec<Entity>>(0)?;
    let ids = inserted_entities
        .iter()
        .map(|d| d.id.clone())
        .collect::<Vec<EntityId>>();

    // FIXME: Content not working only Set
    let mut response = db::get()
        .query("RELATE $device->updates->$entities SET with_polling=$update_with_polling, polling_interval=$update_polling_interval")
        .bind(("device", device_id))
        .bind(("entities", ids))
        .bind((
                  "update_with_polling", updates.with_polling),
        )
        .bind(("update_polling_interval", updates.polling_interval))
        .await?;

    Ok(inserted_entities)
}

pub async fn get_by_device(device_id: DeviceId) -> Result<Vec<Entity>, Rejection> {
    let mut response = db::get()
        .query("array::at((SELECT ->updates->entity as entities FROM $device FETCH entities), 0).entities")
        .bind(("device", device_id))
        .await?;

    Ok(response.take(0)?)
}

pub async fn fetch() -> Result<Vec<Entity>, Rejection> {
    let mut response = db::get().query("SELECT * FROM entity").await?;
    Ok(response.take::<Vec<Entity>>(0)?)
}

pub async fn delete(entity_id: EntityId) -> Result<Vec<Entity>, Rejection> {
    Ok(db::get().delete(entity_id.id_without_brackets()).await?)
}
