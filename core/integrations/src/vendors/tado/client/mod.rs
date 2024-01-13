#![allow(dead_code)]

use std::panic::panic_any;
use std::time::Duration;

pub mod data;
pub mod models;

use chrono::{DateTime, Utc};
use log::debug;
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, REFERER};
use reqwest::{Request, RequestBuilder, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use data::capability::Capability;
use data::device::Device;
use data::states::State;
use data::user::{HomePresence, HomeState, Presence, User};
use data::weather::Weather;
use data::zone::Zone;
use models::{Configuration, Domain, Endpoint};

const CLIENT_ID: &str = "tado-web-app";
const CLIENT_SECRET: &str = "wZaRN7rpjn3FoNyF5IFuxg9uMzYJcvOoQ8QWiIqS3hfk6gLhVlG57j5YNoZL2Rtc";
const TIMEOUT: Duration = Duration::from_secs(10);

static DEFAULT_HEADERS: Lazy<HeaderMap> = Lazy::new(|| {
	let mut headers = HeaderMap::new();
	headers.insert(REFERER, "https://app.tado.com/".parse().unwrap());
	headers
});

#[derive(Clone)]
struct Params<'a> {
	pub endpoint: Endpoint,
	pub domain: Domain,
	pub command: &'a str,
	pub device: &'a str,
}

impl<'a> Default for Params<'a> {
	fn default() -> Self {
		Params {
			endpoint: Endpoint::Api,
			domain: Domain::Home,
			command: "",
			device: "",
		}
	}
}

#[derive(Deserialize, Clone, Debug)]
pub struct AuthenticationConfig {
	pub access_token: String,
	pub refresh_token: String,
	pub expires_in: u64,
	pub scope: String,
	pub token_type: String,
}

#[derive(Serialize)]
struct AuthenticationRequest<'a> {
	pub client_id: &'a str,
	pub client_secret: &'a str,
	pub grant_type: &'a str,
	pub scope: &'a str,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub username: Option<&'a str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub password: Option<&'a str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refresh_token: Option<&'a str>,
}

#[derive(Debug, Clone)]
pub struct Client {
	home_id: u64,
	refresh_at: DateTime<Utc>,
	auth: AuthenticationConfig,
	client: reqwest::Client,
}

impl Client {
	pub async fn new(configuration: Configuration) -> utils::types::Result<Self> {
		let mut builder = reqwest::ClientBuilder::new();
		builder = builder.default_headers(DEFAULT_HEADERS.clone());
		builder = builder.connect_timeout(TIMEOUT);
		builder = builder.https_only(true);

		let client = builder.build()?;

		let username: String = String::from(configuration.username);
		let password: String = String::from(configuration.password);

		let auth: AuthenticationConfig = client
			.post::<Url>(Endpoint::Auth.into())
			.query(&[
				("client_id", CLIENT_ID),
				("client_secret", CLIENT_SECRET),
				("grant_type", "password"),
				("scope", "home.user"),
				("username", &username),
				("password", &password),
			])
			.send()
			.await?
			.json()
			.await?;

		let refresh_at = Utc::now() + Duration::from_secs(auth.expires_in - 30);

		Ok(Client {
			home_id: 0,
			client,
			auth,
			refresh_at,
		})
	}
}

impl Client {
	pub fn use_home(&mut self, home_id: u64) {
		self.home_id = home_id;
	}

	/** Get the current signed in user, with information about homes and devices. */
	pub async fn get_me(&mut self) -> utils::types::Result<User> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Domain::Me,
				command: "",
				device: "",
			},
			false,
		)
		.await
	}

	/** Get the list of devices for the current home. */
	pub async fn get_devices(&mut self) -> utils::types::Result<Vec<Device>> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: "devices",
				device: "",
			},
			false,
		)
		.await
	}

	/** Get the list of zones for the current home. */
	pub async fn get_zones(&mut self) -> utils::types::Result<Vec<Zone>> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: "zones",
				device: "",
			},
			false,
		)
		.await
	}

	pub async fn get_zone_states(&mut self) -> utils::types::Result<Value> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: "zoneStates",
				device: "",
			},
			false,
		)
		.await
	}

	pub async fn get_zone_state(&mut self, zone: &Zone) -> utils::types::Result<State> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: format!("zones/{}/state", zone.id).as_str(),
				device: "",
			},
			false,
		)
		.await
	}

	/** Get the capabilities of a zone. */
	pub async fn get_capabilities(&mut self, zone: &Zone) -> utils::types::Result<Capability> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: format!("zones/{}/capabilities", zone.id).as_str(),
				device: "",
			},
			false,
		)
		.await
	}

	/** Get the state of the selected home. */
	pub async fn get_home_state(&mut self) -> utils::types::Result<HomeState> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: "state",
				device: "",
			},
			false,
		)
		.await
	}

	/** Get the weather for the current home. */
	pub async fn get_weather(&mut self) -> utils::types::Result<Weather> {
		self.request::<_, _>(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: "weather",
				device: "",
			},
			false,
		)
		.await
	}

	/** Get the air comfort for the current home. */
	pub async fn get_air_comfort(&mut self) -> utils::types::Result<Value> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: "airComfort",
				device: "",
			},
			false,
		)
		.await
	}

	/** Get all the users that can control the homes */
	pub async fn get_users(&mut self) -> utils::types::Result<Value> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: "users",
				device: "",
			},
			false,
		)
		.await
	}

	/** Get all the mobile devices that can control the homes */
	pub async fn get_mobile_devices(&mut self) -> utils::types::Result<Value> {
		self.request(
			reqwest::Method::GET,
			"",
			Params {
				endpoint: Default::default(),
				domain: Default::default(),
				command: "mobileDevices",
				device: "",
			},
			false,
		)
		.await
	}

	pub async fn set_presence(&mut self, presence: Presence) -> utils::types::Result<()> {
		match presence {
			Presence::Home | Presence::Away => {
				self.request(
					reqwest::Method::PUT,
					HomePresence {
						home_presence: presence,
					},
					Params {
						endpoint: Default::default(),
						domain: Default::default(),
						command: "presenceLock",
						device: "",
					},
					true,
				)
				.await
			}
			Presence::Auto => {
				self.request(
					reqwest::Method::DELETE,
					"",
					Params {
						endpoint: Default::default(),
						domain: Default::default(),
						command: "presenceLock",
						device: "",
					},
					false,
				)
				.await
			}
		}
	}
}

impl Client {
	fn get_url(params: Params, home_id: Option<u64>) -> String {
		match params.endpoint {
			Endpoint::Mobile => format!("{}{}", params.endpoint, params.command),
			_ => match params.domain {
				Domain::Devices => format!(
					"{}{}/{}/{}",
					params.endpoint, params.domain, params.device, params.command
				),
				Domain::Me => format!("{}{}", params.endpoint, params.domain),
				_ => match home_id {
					None => panic_any("No home has been assigned to this client"),
					Some(id) => {
						format!("{}{}/{}/{}", params.endpoint, params.domain, id, params.command)
					}
				},
			},
		}
	}

	async fn request<'a, Payload, Output>(
		&mut self,
		method: reqwest::Method,
		payload: Payload,
		params: Params<'a>,
		serialize: bool,
	) -> utils::types::Result<Output>
	where
		Payload: Serialize,
		Output: for<'b> Deserialize<'b>,
	{
		if Utc::now() > self.refresh_at {
			self.refresh_token().await?;
		}

		let request = Request::new(
			method,
			Client::get_url(params.clone(), Some(self.home_id)).parse().unwrap(),
		);
		let mut builder = RequestBuilder::from_parts(self.client.clone(), request)
			.query(&[("ngsw-bypass", "true")])
			.bearer_auth(&self.auth.access_token);

		if serialize {
			builder = builder.json(&payload);
		} else {
			// FIXME: Inefficient conversion, find alternative with traits specification
			builder = builder.body(serde_json::to_string(&payload).unwrap());
		}

		let response = builder.send().await?;
		Ok(response.json().await?)
	}

	async fn refresh_token(&mut self) -> utils::types::Result<()> {
		debug!("Refreshing auth token");

		let response = self
			.client
			.post::<Url>(Endpoint::Auth.into())
			.query(&[
				("client_id", CLIENT_ID),
				("client_secret", CLIENT_SECRET),
				("grant_type", "refresh_token"),
				("scope", "home.user"),
				("refresh_token", &self.auth.refresh_token),
			])
			.send()
			.await?;

		self.auth = response.json().await?;

		Ok(())
	}
}
