use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

mod methods;
mod request;
mod response;

pub mod client;
mod worker;

type Output = Result<Value, RpcError>;

#[derive(Debug, Error)]
pub enum RpcError {
	#[error("Method not found")]
	MethodNotFound,
	#[error("Invalid ID provided")]
	InvalidId,
	#[error("Internal server error")]
	InternalServerError,
	#[error("Processing error")]
	ProcessingError(#[from] err::Error),
	#[error("Serialization error")]
	SerializationError(#[from] serde_json::Error),
	#[error("Missing parameter: {0}")]
	MissingParameter(&'static str),
	#[error("Invalid parameter: {0}")]
	InvalidParameter(&'static str),
}

impl Serialize for RpcError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}
