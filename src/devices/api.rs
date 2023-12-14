use serde_json::Value;

use crate::db;
use crate::devices::models::{Device, DeviceId};
use crate::errors::Error;
use crate::integrations::ComponentId;

pub async fn create(
    device: Device,
    component_id: ComponentId,
) -> miette::Result<Vec<Value>, Error> {
    create_multiple(vec![device], component_id).await
}

pub async fn create_multiple(
    devices: Vec<Device>,
    component_id: ComponentId,
) -> miette::Result<Vec<Value>, Error> {
    let mut insert_response = db::get()
        .query("INSERT INTO device $devices")
        .bind(("devices", devices))
        .await?;

    // Only relate inserted devices
    let inserted_devices = insert_response.take::<Vec<Device>>(0)?;
    let ids = inserted_devices
        .iter()
        .map(|d| d.id.clone())
        .collect::<Vec<DeviceId>>();

    let mut response = db::get()
        .query("RELATE $component->controls->$devices")
        .bind(("component", component_id))
        .bind(("devices", ids))
        .await?;

    Ok(response.take(0)?)
}

pub async fn get(device_id: DeviceId) -> miette::Result<Option<Device>, Error> {
    let mut response = db::get()
        .query("SELECT * FROM ONLY device:$id")
        .bind(("id", device_id))
        .await?;

    Ok(response.take::<Option<Device>>(0)?)
}

pub async fn list_all() -> miette::Result<Vec<Device>, Error> {
    let mut response = db::get().query("SELECT * FROM device").await?;
    Ok(response.take::<Vec<Device>>(0)?)
}
