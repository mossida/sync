use std::{path::PathBuf, sync::OnceLock};

use crate::config::Configuration;

use ::config::{Config, Environment, File};
use err::Error;

mod config;
mod database;
mod secrets;

pub const PKG_NAME: &str = "sync";

pub const CONFIG_FILE: &str = PKG_NAME;

pub const HELP_URL: &str = "https://docs.mossida.com/sync";

static CONFIG: OnceLock<Configuration> = OnceLock::new();

pub fn init(file: Option<PathBuf>) -> Result<(), Error> {
	let config_file = file.unwrap_or_else(|| PathBuf::from(CONFIG_FILE));
	let config = Config::builder()
		.add_source(File::from(config_file).required(false))
		.add_source(Environment::with_prefix(PKG_NAME))
		.build()?
		.try_deserialize()?;

	let _ = CONFIG.set(config);

	Ok(())
}

pub fn get() -> &'static Configuration {
	CONFIG.get().unwrap()
}
