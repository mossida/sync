use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
	id: String,
	method: String,
	params: HashMap<String, serde_json::Value>,
}
