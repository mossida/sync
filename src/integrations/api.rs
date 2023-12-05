use crate::api::rejections::Rejection;
use crate::db;
use crate::devices::models::Device;
use crate::integrations::Component;
use uuid::Uuid;

pub async fn create(component: Component) -> Result<Vec<Component>, Rejection> {
    Ok(db::get()
        .create("component")
        .content::<Component>(component)
        .await?)
}

pub async fn get(component_id: Uuid) -> Result<Option<Device>, Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM ONLY component:$id")
        .bind(("id", component_id))
        .await?;

    Ok(response.take::<Option<Device>>(0)?)
}

pub async fn list_all() -> Result<Vec<Component>, Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM component ORDER BY priority DESC")
        .await?;
    Ok(response.take::<Vec<Component>>(0)?)
}
