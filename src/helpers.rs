use std::sync::Arc;

use surrealdb::Error;
use tokio::sync::Mutex;

use crate::types::SyncObject;

pub struct Helper {}

impl Helper {
    pub fn sync<T>(data: T) -> SyncObject<T> {
        Arc::new(Mutex::new(data))
    }

    pub fn reject_db(_: Error) -> warp::reject::Rejection {
        warp::reject()
    }
}
