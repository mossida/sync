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
}

pub fn log<T>(result: types::Result<T>) {
    match result {
        Ok(_) => (),
        Err(e) => log::error!("{}", e),
    }
}
