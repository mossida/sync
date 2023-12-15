use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: ConfigDatabase,
    pub general: ConfigGeneral,
    pub secrets: ConfigSecrets,
}

#[derive(Debug, Deserialize)]
pub struct ConfigDatabase {
    pub host: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigGeneral {
    pub log_level: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigSecrets {
    pub path: String,
}
