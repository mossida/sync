use config::Config;
use once_cell::sync::Lazy;
use serde::Deserialize;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Configuration {
	pub database: DatabaseConfiguration,
	pub secrets: SecretsConfiguration,
}

#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct SecretsConfiguration {
	pub path: String,
}

#[derive(Deserialize, Clone, Zeroize, ZeroizeOnDrop)]
pub struct DatabaseConfiguration {
	pub endpoint: String,
	pub namespace: String,
	pub database: String,
	pub username: Option<String>,
	pub password: Option<String>,
}

// FIXME: Handle errors correctly
static CONFIG: Lazy<Configuration> = Lazy::new(|| {
	Config::builder()
		.add_source(config::File::with_name("config"))
		.build()
		.unwrap()
		.try_deserialize::<Configuration>()
		.unwrap()
});

pub fn get() -> &'static Configuration {
	&CONFIG
}
