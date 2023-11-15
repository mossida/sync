use std::future::IntoFuture;

use futures::executor::block_on;

use crate::events::models::*;
use crate::models::Record;
use crate::DB;

#[allow(dead_code)]
pub fn send(event: &Event) -> Result<Vec<Record>, surrealdb::Error> {
    return block_on(DB.create("event").content(event).into_future());
}

#[allow(dead_code)]
pub fn listen(event_type: &EventType) {}
