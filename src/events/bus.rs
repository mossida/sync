use std::future::IntoFuture;

use crate::db;
use futures::executor::block_on;

use crate::events::models::*;
use crate::models::Record;

#[allow(dead_code)]
pub fn send(event: &Event) -> Result<Vec<Record>, surrealdb::Error> {
    return block_on(db::get().create("event").content(event).into_future());
}

#[allow(dead_code)]
pub fn listen(event_type: &EventType) {}
