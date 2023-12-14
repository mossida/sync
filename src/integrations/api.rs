use uuid::Uuid;

use crate::db;
use crate::devices::models::Device;
use crate::errors::Error;
use crate::integrations::Component;

pub async fn create(component: Component) -> miette::Result<Vec<Component>, Error> {
    Ok(db::get()
        .create("component")
        .content::<Component>(component)
        .await?)
}

pub async fn get(component_id: Uuid) -> miette::Result<Option<Device>, Error> {
    let mut response = db::get()
        .query("SELECT * FROM ONLY component:$id")
        .bind(("id", component_id))
        .await?;

    Ok(response.take::<Option<Device>>(0)?)
}

pub async fn list_all() -> miette::Result<Vec<Component>, Error> {
    let mut response = db::get()
        .query("SELECT * FROM component ORDER BY priority DESC")
        .await?;
    Ok(response.take::<Vec<Component>>(0)?)
}
