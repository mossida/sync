use serde::Deserialize;

#[derive(Deserialize)]
pub struct Secrets {
	pub key: String,
	pub path: String,
}
