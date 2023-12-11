use crate::api::rejections::Rejection;
use crate::db;
use crate::entities::models::EntityId;
use crate::integrations::classes::Attributes;
use crate::states::models::state::{State, StateId};

pub async fn set_state(state: State) -> Result<Option<State>, Rejection> {
    let mut response = db::get()
        .query("INSERT INTO state $state")
        .bind(("state", state))
        .await?;

    Ok(response.take(0)?)
}

pub async fn set_attributes(
    state_id: StateId,
    attributes: Attributes,
) -> Result<Option<State>, Rejection> {
    let mut response = db::get()
        .query("UPDATE $state SET attributes=$attributes")
        .bind(("state", state_id))
        .bind(("attributes", attributes))
        .await?;

    Ok(response.take(0)?)
}

pub async fn get_state_of_entity(entity_id: EntityId) -> Result<Option<State>, Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM state WHERE entity_id=$entity_id")
        .bind(("entity_id", entity_id))
        .await?;

    Ok(response.take(0)?)
}
