use std::path::Path;
use std::sync::OnceLock;

use log::warn;
use securestore::{KeySource, SecretsManager};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::configuration;

static SECRETS: OnceLock<SecretsManager> = OnceLock::new();

pub fn create() -> SecretsManager {
    let configuration = configuration::get();

    SecretsManager::load(
        &configuration.secrets.path,
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

#[derive(Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct Secret(String, Option<String>);

impl Secret {
    pub fn new<T: Into<String>>(secret_key: T) -> Self {
        Self(secret_key.into(), None)
    }

    pub fn create_with_value<T: Into<String>>(secret_key: T, secret_value: T) -> Self {
        let key = secret_key.into();
        let value = secret_value.into();

        let mut manager = create();

        manager.set(&key, &*value);
        manager.save().unwrap();

        init();
        Self(key, Some(value))
    }
}

/*
FIXME: The drop is not handled correctly
the option will be removed from the memory but not the String
 */
impl From<Secret> for String {
    fn from(secret: Secret) -> Self {
        secret.1.clone().unwrap_or_default()
    }
}

impl Serialize for Secret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Secret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secret_key = String::deserialize(deserializer)?;
        let secret_value = get().get(&secret_key);

        if secret_value.is_err() {
            warn!("Secret deserialization cannot be done, the secret key doesn't exist")
        }

        Ok(Secret(
            secret_key.clone(),
            Some(secret_value.unwrap_or_default()),
        ))
    }
}
