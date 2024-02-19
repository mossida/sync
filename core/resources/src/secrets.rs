use std::path::Path;
use std::sync::OnceLock;

use log::warn;
use schemars::gen::SchemaGenerator;
use schemars::schema::{Schema, SchemaObject};
use schemars::JsonSchema;
use securestore::{KeySource, SecretsManager};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zeroize::{Zeroize, ZeroizeOnDrop};

static SECRETS: OnceLock<SecretsManager> = OnceLock::new();

pub fn create() -> SecretsManager {
	let configuration = &cnf::get().secrets;

	SecretsManager::load(&configuration.path, KeySource::Path(Path::new(&configuration.key)))
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

impl JsonSchema for Secret {
	fn schema_name() -> String {
		"Secret".to_owned()
	}

	fn json_schema(gen: &mut SchemaGenerator) -> Schema {
		let mut object: SchemaObject = String::json_schema(gen).into();
		object.format = Some("secret".to_owned());
		object.into()
	}
}

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

		Ok(Secret(secret_key.clone(), Some(secret_value.unwrap_or_default())))
	}
}
