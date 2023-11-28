use crate::api::rejections::Rejection;
use crate::db;
use crate::devices::models::{Device, DeviceId};

pub async fn get(device_id: DeviceId) -> Result<Option<Device>, Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM ONLY device:$id")
        .bind(("id", device_id))
        .await?;

    Ok(response.take::<Option<Device>>(0)?)
}

pub async fn list_all() -> Result<Vec<Device>, Rejection> {
    let mut response = db::get().query("SELECT * FROM device").await?;
    Ok(response.take::<Vec<Device>>(0)?)
}
