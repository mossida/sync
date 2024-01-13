use std::fmt::{Display, Formatter};

use reqwest::Url;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use resources::secrets::Secret;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Configuration {
	pub username: Secret,
	pub password: Secret,
}

#[derive(Clone, PartialEq, Default)]
pub enum Domain {
	#[default]
	Home,
	Devices,
	Me,
}

impl Display for Domain {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", <Domain as Into<&str>>::into(self.clone()))
	}
}

impl From<Domain> for &str {
	fn from(value: Domain) -> Self {
		match value {
			Domain::Home => "homes",
			Domain::Devices => "devices",
			Domain::Me => "me",
		}
	}
}

#[derive(Clone, PartialEq, Default)]
pub enum Endpoint {
	#[default]
	Api,
	Mobile,
	Eqi,
	Auth,
}

impl Display for Endpoint {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", <Endpoint as Into<&str>>::into(self.clone()))
	}
}

impl From<Endpoint> for &str {
	fn from(endpoint: Endpoint) -> Self {
		match endpoint {
			Endpoint::Api => "https://my.tado.com/api/v2/",
			Endpoint::Mobile => "https://my.tado.com/mobile/1.9/",
			Endpoint::Eqi => "https://energy-insights.tado.com/api/",
			Endpoint::Auth => "https://auth.tado.com/oauth/token",
		}
	}
}

impl From<Endpoint> for Url {
	fn from(endpoint: Endpoint) -> Self {
		match endpoint {
			Endpoint::Api => Url::parse("https://my.tado.com/api/v2/").unwrap(),
			Endpoint::Mobile => Url::parse("https://my.tado.com/mobile/1.9/").unwrap(),
			Endpoint::Eqi => Url::parse("https://energy-insights.tado.com/api/").unwrap(),
			Endpoint::Auth => Url::parse("https://auth.tado.com/oauth/token").unwrap(),
		}
	}
}
