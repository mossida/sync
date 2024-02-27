use err::Error;
use serde_json::Value;

pub struct Response {
	pub id: u64,
	pub result: Result<Value, Error>,
}
