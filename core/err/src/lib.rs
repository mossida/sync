use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(help("https:://docs.mossida.com/sync"))]
pub enum Error {
	#[error("There is an issue with the configuration")]
	#[diagnostic(code(sync::invalid_config))]
	ConfigError(#[from] ::config::ConfigError),
	#[error("Something went wrong with the database")]
	#[diagnostic(code(sync::database_error))]
	DatabaseError(#[from] surrealdb::Error),
	#[error("Something went wrong during the migration")]
	#[diagnostic(code(sync::migration_error))]
	MigrationError(#[from] MigrationError),
}

#[derive(Debug, Diagnostic, Error)]
#[error("{message}")]
pub struct MigrationError {
	pub message: String,
}
