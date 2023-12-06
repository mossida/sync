use std::fmt::{Display, Formatter};


use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::secrets::models::Secret;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub username: Secret,
    pub password: Secret,
}

#[derive(Clone, PartialEq, Default)]
pub enum Domain {
    #[default]
    HOME,
    DEVICES,
    ME,
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Domain as Into<&str>>::into(self.clone()))
    }
}

impl From<Domain> for &str {
    fn from(value: Domain) -> Self {
        match value {
            Domain::HOME => "homes",
            Domain::DEVICES => "devices",
            Domain::ME => "me",
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum Endpoint {
    #[default]
    API,
    MOBILE,
    EQI,
    AUTH,
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Endpoint as Into<&str>>::into(self.clone()))
    }
}

impl From<Endpoint> for &str {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint {
            Endpoint::API => "https://my.tado.com/api/v2/",
            Endpoint::MOBILE => "https://my.tado.com/mobile/1.9/",
            Endpoint::EQI => "https://energy-insights.tado.com/api/",
            Endpoint::AUTH => "https://auth.tado.com/oauth/token",
        }
    }
}

impl From<Endpoint> for Url {
    fn from(endpoint: Endpoint) -> Self {
        match endpoint {
            Endpoint::API => Url::parse("https://my.tado.com/api/v2/").unwrap(),
            Endpoint::MOBILE => Url::parse("https://my.tado.com/mobile/1.9/").unwrap(),
            Endpoint::EQI => Url::parse("https://energy-insights.tado.com/api/").unwrap(),
            Endpoint::AUTH => Url::parse("https://auth.tado.com/oauth/token").unwrap(),
        }
    }
}
