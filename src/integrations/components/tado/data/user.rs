use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HomePresence {
    #[serde(rename = "homePresence")]
    pub home_presence: Presence,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Presence {
    AWAY,
    HOME,
    AUTO,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub username: String,
    pub id: String,
    pub homes: Vec<Home>,
    pub locale: String,
    #[serde(rename = "mobileDevices")]
    pub mobile_devices: Vec<MobileDevice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Home {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct HomeState {
    pub presence: String,
    #[serde(rename = "presenceLocked")]
    pub presence_locked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobileDevice {
    pub id: u64,
    pub name: String,
    #[serde(skip_deserializing)]
    pub settings: String,
    #[serde(rename = "deviceMetadata", skip_deserializing)]
    pub device_metadata: String,
}
