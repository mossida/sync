use miette::Result;
use surreal_id::NewId;

use crate::db;
use crate::devices::models::DeviceId;
use crate::entities::models::{Entity, EntityId, Updates};
use crate::errors::Error;
use crate::integrations::classes::{Attributes, Class};
use crate::scheduler::definitions::InterfaceName;

pub async fn merge_attributes(
    entity_id: &EntityId,
    attributes: &Attributes,
) -> Result<Option<Entity>, Error> {
    let mut response = db::get()
        .query("UPDATE $entity SET state.attributes = $attributes")
        .bind(("entity", entity_id))
        .bind(("attributes", attributes))
        .await?;

    Ok(response.take(0)?)
}

pub async fn set_state(
    entity_id: &EntityId,
    state: serde_json::Value,
) -> Result<Option<Entity>, Error> {
    let mut response = db::get()
        .query("UPDATE $entity SET state.state = $state")
        .bind(("entity", entity_id))
        .bind(("state", state))
        .await?;

    Ok(response.take(0)?)
}

pub async fn create(
    entities: Vec<Entity>,
    device_id: DeviceId,
    updates: Updates,
) -> Result<Vec<Entity>, Error> {
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
    let _ = db::get()
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

pub async fn fetch_by_interface(interface_name: InterfaceName) -> Result<Option<Entity>, Error> {
    let mut response = db::get()
        .query("SELECT * FROM entity WHERE interface = $interface")
        .bind(("interface", interface_name))
        .await?;

    Ok(response.take(0)?)
}

pub async fn fetch_by_device(device_id: DeviceId) -> Result<Vec<Entity>, Error> {
    let mut response = db::get()
        .query("(SELECT VALUE ->updates->entity FROM ONLY $device LIMIT 1 FETCH entities)")
        .bind(("device", device_id))
        .await?;

    Ok(response.take(0)?)
}

pub async fn fetch_by_device_and_class(
    device_id: DeviceId,
    class: Class,
) -> Result<Vec<Entity>, Error> {
    let mut response = db::get()
        .query(
            "SELECT * FROM (SELECT VALUE ->updates->entity FROM ONLY $device LIMIT 1 FETCH entity) WHERE class=$class",
        )
        .bind(("device", device_id))
        .bind(("class", class))
        .await?;

    Ok(response.take(0)?)
}

pub async fn fetch_all() -> Result<Vec<Entity>, Error> {
    let mut response = db::get().query("SELECT * FROM entity").await?;
    Ok(response.take::<Vec<Entity>>(0)?)
}

pub async fn delete(entity_id: EntityId) -> Result<Vec<Entity>, Error> {
    Ok(db::get().delete(entity_id.id_without_brackets()).await?)
}
