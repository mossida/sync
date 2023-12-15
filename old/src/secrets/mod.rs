use std::path::Path;
use std::sync::OnceLock;

use securestore::{KeySource, SecretsManager};

use crate::CONFIG;

pub(crate) mod models;

static SECRETS: OnceLock<SecretsManager> = OnceLock::new();

pub fn create() -> SecretsManager {
    SecretsManager::load(
        &CONFIG.secrets.path,
        KeySource::Path(Path::new("secrets.key")),
    )
    .unwrap()
}

pub fn init() {
    let _ = SECRETS.set(create());
}

pub fn get() -> &'static SecretsManager {
    SECRETS.get().unwrap()
}
