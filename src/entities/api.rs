use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use surreal_id::NewId;

use crate::api::rejections::Rejection;
use crate::db;
use crate::devices::models::DeviceId;
use crate::entities::models::{Entity, EntityId, Updates};

// FIXME: Return types are inconsistent
pub async fn create<T>(
    entity: Entity<T>,
    device_id: DeviceId,
    updates: Updates,
) -> Result<Vec<Value>, Rejection>
where
    T: Serialize + DeserializeOwned,
{
    create_multiple(vec![entity], device_id, updates).await
}

pub async fn create_multiple<T>(
    entities: Vec<Entity<T>>,
    device_id: DeviceId,
    updates: Updates,
) -> Result<Vec<Value>, Rejection>
where
    T: Serialize + DeserializeOwned,
{
    let mut insert_response = db::get()
        .query("INSERT INTO entity $entities")
        .bind(("entities", entities))
        .await?;

    // Only relate inserted entities
    let inserted_entities = insert_response.take::<Vec<Entity<T>>>(0)?;
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

    Ok(response.take(0)?)
}

pub async fn set_state<T>(entity_id: &EntityId, state: T) -> Result<Option<Value>, Rejection>
where
    T: Serialize + DeserializeOwned,
{
    let mut response = db::get()
        .query("UPDATE $entity SET state=$state")
        .bind(("entity", entity_id))
        .bind(("state", state))
        .await?;

    Ok(response.take(0)?)
}

pub async fn get_by_device<T>(device_id: DeviceId) -> Result<Vec<Entity<T>>, Rejection>
where
    T: DeserializeOwned,
{
    let mut response = db::get()
        .query("array::at((SELECT ->updates->entity as entities FROM $device FETCH entities), 0).entities")
        .bind(("device", device_id))
        .await?;

    Ok(response.take(0)?)
}

pub async fn fetch<T>() -> Result<Vec<Entity<T>>, Rejection>
where
    T: DeserializeOwned,
{
    let mut response = db::get().query("SELECT * FROM entity").await?;
    Ok(response.take::<Vec<Entity<T>>>(0)?)
}

pub async fn delete(entity_id: EntityId) -> Result<Vec<Entity<Value>>, Rejection> {
    Ok(db::get().delete(entity_id.id_without_brackets()).await?)
}
