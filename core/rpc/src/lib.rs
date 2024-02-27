use std::pin::Pin;

use futures::Future;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

mod methods;
mod request;
mod response;

pub mod client;
mod worker;

type Output = Result<Value, RpcError>;
type IntoFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;

#[derive(Debug, Serialize, Error)]
pub enum RpcError {
	#[error("Method not found")]
	MethodNotFound,
}
