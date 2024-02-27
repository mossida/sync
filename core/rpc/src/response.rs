use crate::Output;
use axum::extract::ws::Message;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
	pub id: u64,
	pub result: Output,
}

pub trait IntoResponse {
	fn into_response(self, id: u64) -> Response;
}

impl IntoResponse for Output {
	fn into_response(self, id: u64) -> Response {
		Response {
			id,
			result: self,
		}
	}
}

impl TryInto<Message> for Response {
	type Error = serde_json::Error;

	fn try_into(self) -> Result<Message, Self::Error> {
		serde_json::to_string(&self).map(Message::Text)
	}
}
