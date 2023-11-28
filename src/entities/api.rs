use crate::api::rejections::{Rejection, RejectionCode};
use crate::db;
use crate::entities::models::{Entity, EntityId};

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

pub async fn delete(entity_id: EntityId) -> Result<Option<Entity>, Rejection> {
    Ok(db::get().delete(("entity", entity_id)).await?)
}
