use miette::Diagnostic;

use thiserror::Error;

pub use miette::Result;

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
	#[error("Something went wrong during the interface")]
	#[diagnostic(code(sync::interface_error))]
	InterfaceError(#[from] serde_json::Error),
	#[error("Something went wrong during the IO")]
	#[diagnostic(code(sync::io_error))]
	IoError(#[from] std::io::Error),
	#[error("Something went wrong during an actor spawn")]
	#[diagnostic(code(sync::actor_error))]
	ActorError(#[from] ractor::SpawnErr),
	#[error("{0}")]
	#[diagnostic(code(sync::custom_error))]
	CustomError(String),
}

#[derive(Debug, Diagnostic, Error)]
#[error("{message}")]
pub struct MigrationError {
	pub message: String,
}
