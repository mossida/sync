use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub database: ConfigDatabase,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigDatabase {
    pub host: String,
}

/*impl Default for Config {
    fn default() -> Config {
        Config {
            db_endpoint: "localhost".to_string(),
        }
    }
}*/
