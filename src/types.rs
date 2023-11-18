use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

//pub type ApiResponse = impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone;

pub type SyncObject<T> = Arc<Mutex<T>>;

pub type SyncMap<K, V> = HashMap<K, SyncObject<V>>;
