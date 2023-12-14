use std::error;

use miette::Diagnostic;
use ractor::SpawnErr;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use thiserror::Error;
use warp::reject::Reject;

// FIXME: Refactor
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("Unknown error")]
    #[diagnostic(code(sync::unknown))]
    Unknown,
    #[error("Database error: {0}")]
    #[diagnostic(code(sync::database::query))]
    Database(#[from] surrealdb::Error),
    #[error("Id conversion has got an error: {0}")]
    #[diagnostic(code(sync::database::id))]
    DatabaseId(#[from] surreal_id::IdError),
    #[error("Client error: {0}")]
    Client(#[from] reqwest::Error),
    #[error("Interface error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Configuration error: {0}")]
    Migration(String),
    #[error("Configuration error: {0}")]
    Actor(#[from] SpawnErr),
    #[error("Configuration error: {0}")]
    Interface(String),
}

impl Reject for Error {}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("error", &self.to_string())?;
        map.end()
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum MainError {
    #[error(transparent)]
    Database {
        #[from]
        source: surrealdb::Error,
    },
}
