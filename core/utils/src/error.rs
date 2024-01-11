use thiserror::Error;

use crate::types;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database responded with an error: {0}")]
    Database(#[from] surrealdb::Error),
    #[error("Migration error: {0}")]
    Migration(String),
    #[error("Integration cannot start: {0}")]
    Integration(#[from] ractor::SpawnErr),
    #[error("Something wrong happened when trying to make a request: {0}")]
    HttpClient(#[from] reqwest::Error),
    #[error("Something wrong with the serialization")]
    InterfaceError(#[from] serde_json::Error),
    #[error("Something wrong with the IO: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn log<T>(result: types::Result<T>) {
    match result {
        Ok(_) => (),
        Err(e) => log::error!("{}", e),
    }
}
