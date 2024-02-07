use futures::{Stream, TryStreamExt};
use resources::database;
use serde::{Deserialize, Serialize};
use surrealdb::{
	sql::{Datetime, Thing},
	Notification,
};

use utils::types::Result;

pub const RESOURCE: &str = "event";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EventData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
	StateChange,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
	pub name: EventType,
	pub data: EventData,
	pub source: Thing,
	pub occured_at: Datetime,
}

impl Event {
	pub async fn live() -> Result<impl Stream<Item = Result<Notification<Event>>>> {
		let client = database::get();
		Ok(client.select(RESOURCE).live().await?.map_err(|err| err.into()))
	}
}
