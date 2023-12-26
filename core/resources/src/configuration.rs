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

#[derive(Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct DatabaseConfiguration {
    pub host: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub namespace: String,
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
