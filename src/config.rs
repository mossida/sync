use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: ConfigDatabase,
    pub general: ConfigGeneral,
}

#[derive(Debug, Deserialize)]
pub struct ConfigDatabase {
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigGeneral {
    pub log_level: String,
}

/*impl Default for Config {
    fn default() -> Config {
        Config {
            db_endpoint: "localhost".to_string(),
        }
    }
}*/
