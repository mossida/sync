use crate::types::SyncObject;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Helper {}

impl Helper {
    pub fn sync<T>(data: T) -> SyncObject<T> {
        Arc::new(Mutex::new(data))
    }
}
