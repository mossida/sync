use crate::config::Configuration;

use ::config::{Config, Environment, File};
use once_cell::sync::Lazy;

mod config;
mod database;
mod secrets;

pub const PKG_NAME: &str = "sync";

pub const CONFIG_FILE: &str = PKG_NAME;

pub static CONFIG: Lazy<Configuration> = Lazy::new(|| {
	Config::builder()
		.add_source(File::with_name(CONFIG_FILE).required(false))
		.add_source(Environment::with_prefix(PKG_NAME))
		.build()
		.unwrap()
		.try_deserialize()
		.unwrap()
});
