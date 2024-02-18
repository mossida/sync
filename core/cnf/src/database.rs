use serde::Deserialize;

#[derive(Deserialize)]
pub struct Database {
	pub endpoint: String,
	pub namespace: String,
	pub database: String,
	pub username: Option<String>,
	pub password: Option<String>,
}
