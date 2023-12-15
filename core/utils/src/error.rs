use crate::types;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database responded with an error: {0}")]
    Database(#[from] surrealdb::Error),
    #[error("Migration error")]
    Migration(String),
}

pub fn log<T>(result: types::Result<T>) {
    match result {
        Ok(_) => (),
        Err(e) => log::error!("{}", e),
    }
}
