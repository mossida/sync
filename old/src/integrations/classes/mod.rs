use derive_more::Display;
use hashbrown::HashMap;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod climate;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Attributes(HashMap<String, Value>);

#[derive(Debug, Display, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    Climate,
}

impl Attributes {
    pub fn new<T>(vec: Vec<T>) -> Attributes
    where
        T: Serialize + DeserializeOwned,
    {
        let object = serde_json::to_value(vec).unwrap();
        let mut map = HashMap::new();

        if let Value::Array(array) = object {
            for value in array.iter() {
                if let Value::Object(object) = value {
                    map.extend(object.clone());
                }
            }
        }

        Attributes(map)
    }

    pub fn merge(&mut self, other: &Attributes) {
        self.0.extend(other.0.clone());
    }

    pub fn merge_vec<T>(&mut self, vec: Vec<T>)
    where
        T: Serialize + DeserializeOwned,
    {
        self.0.extend(Attributes::new(vec).0);
    }
}
