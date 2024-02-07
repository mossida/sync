use futures::{future, Stream, StreamExt};
use resources::database;
use serde::{Deserialize, Serialize};
use surrealdb::{
	method::Stream as LiveStream,
	sql::{Datetime, Thing},
	Notification,
};
use tracing::error;

use utils::types::Result;

pub const RESOURCE: &str = "event";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum EventData {
	Empty {},
}

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
	pub occurred_at: Datetime,
}

impl Event {
	pub async fn live() -> Result<impl Stream<Item = Notification<Event>>> {
		let client = database::get();
		let stream: LiveStream<_, Vec<Event>> = client.select(RESOURCE).live().await?;

		Ok(stream.filter_map(|result| match result {
			Ok(notification) => future::ready(Some(notification)),
			Err(err) => {
				error!("{}", err);
				future::ready(None)
			}
		}))
	}
}
