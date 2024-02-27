use std::marker::PhantomData;

use attribute::{Attribute, Attributes};
use dashmap::DashMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use dbm::{
	resource::{Base, Resource},
	Id,
};
use serde_json::Value;

pub mod attribute;
pub mod device;

pub mod class;

/// This trait represents the class of an entity.
pub trait Class: Send + Sync {
	const NAME: &'static str;
}

/// This trait represents the state of an entity.
/// Must implement serialization and deserialization in order to be
/// stored in the database.
///
/// Also requires Default, to be able to create a new instance of the state.
/// and since it represents a state, it should have a default one.
///
/// Usually the default state is `Unkown`
pub trait State: Default + Serialize + DeserializeOwned + Send + Sync {
	/// Returns the next state of the entity.
	fn next(&self) -> Self {
		Default::default()
	}
}

/// Represents an entity with a specific class and state.
#[derive(Serialize, Deserialize)]
pub struct Entity<C, S>
where
	C: Class,
	S: State,
{
	id: dbm::Id,

	r#type: String,

	/// Extra attributes for the entity.
	/// They add useful information about
	/// the current state of the entity.
	extra: Attributes,

	/// Indicates the state of the entity.
	/// The state is a custom struct/enum that implements the State trait.
	#[serde(bound = "")]
	state: S,

	/// Indicates the class of the entity.
	/// The class is a custom struct/enum that implements the Class trait.
	/// It is used to identify the type of the entity.
	#[serde(skip)]
	marker: PhantomData<C>,

	/// Other properties for the entity can be useful for custom
	/// custom widgets, or custom logic for the entity.
	#[serde(flatten)]
	properties: DashMap<String, Value>,
}

impl<C, S> Entity<C, S>
where
	C: Class,
	S: State,
{
	/// Creates a new instance of the Entity struct with default values for the class, state, and extra fields.
	pub fn new(id: Option<Id>) -> Self {
		Entity {
			id: id.unwrap_or_default(),
			r#type: C::NAME.to_string(),

			marker: Default::default(),
			state: Default::default(),
			extra: Default::default(),

			properties: Default::default(),
		}
	}

	/// Sets the state of the entity.
	pub fn set_state(&mut self, state: S) {
		self.state = state;
	}

	/// Transitions the entity to the next state.
	pub fn next_state(&mut self) {
		self.state = self.state.next();
	}

	/// Sets an attribute for the entity.
	pub fn set_attribute<A>(&self, value: A)
	where
		A: Into<Attribute>,
	{
		self.extra.insert(value.into());
	}
}

impl<C, S> Base for Entity<C, S>
where
	C: Class,
	S: State,
{
	const RESOURCE: &'static str = "entity";
}

/// Implements the Resource trait for the Entity struct.
/// allows handling the entity as a resource in the database.
/// And adds useful methods for handling the corresponding records.
impl<C, S> Resource for Entity<C, S>
where
	C: Class,
	S: State,
{
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}
