use std::{sync::OnceLock, thread};

use rumqttd::Broker;
use tracing::error;

pub static BROKER: OnceLock<Mutex<Broker>> = OnceLock::new();

pub fn init() {
	let config = cnf::get().mqtt.clone();

	let _ = thread::spawn(move || {
		let mut broker = Broker::new(config);
		let result = broker.start();

		if let Err(e) = result {
			error!("MQTT Broker got an error: {:?}", e);
		}
	});
}
