use crate::api::rejections::{Rejection, RejectionCode};
use crate::db;
use crate::devices::models::Device;
use crate::entities::models::{Entity, EntityId};
use surreal_id::NewId;

pub async fn create(entity: Entity) -> Result<Vec<Entity>, Rejection> {
    Ok(db::get().create("entity").content::<Entity>(entity).await?)
}

pub async fn fetch() -> Result<Vec<Entity>, Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM entity")
        .await
        .map_err(|err| Rejection {
            reason: RejectionCode::DATABASE,
            message: err.to_string(),
        })?;

    response.take::<Vec<Entity>>(0).map_err(|err| Rejection {
        reason: RejectionCode::DATABASE,
        message: err.to_string(),
    })
}

pub async fn delete(entity_id: EntityId) -> Result<Vec<Entity>, Rejection> {
    Ok(db::get().delete(entity_id.id_without_brackets()).await?)
}
