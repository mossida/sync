use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use surrealdb::opt::RecordId;
use surrealdb::sql::{Id, Thing};

use crate::integrations::ComponentId;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceId(RecordId);

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: DeviceId,
    pub name: String,
    pub serial: String,
    pub model: String,
    pub manufacturer: String,
    pub sw_version: String,
    pub hw_version: String,
    pub entities: Vec<Thing>,
    pub managed_by: ComponentId,
}

impl NewId for DeviceId {
    const TABLE: &'static str = "device";

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        DeviceId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }

    fn get_inner_string(&self) -> String {
        self.0.id.to_string()
    }
}
