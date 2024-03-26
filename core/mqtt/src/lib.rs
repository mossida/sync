use std::{
	error::Error,
	sync::{Arc, Mutex, OnceLock},
	thread,
};

use rumqttd::local::{LinkRx, LinkTx};
use tracing::error;

pub use rumqttd::*;

pub type Client = (LinkTx, LinkRx);

pub static BROKER: OnceLock<Arc<Mutex<Broker>>> = OnceLock::new();

pub fn client(name: &str) -> Result<Client, Box<dyn Error>> {
	let reference = BROKER.get().ok_or("Broker not available")?;
	let broker = reference.lock()?;

	Ok(broker.link(name)?)
}

pub fn init() {
	let config = cnf::get().mqtt.clone();
	let broker = Broker::new(config);

	let _ = BROKER.set(Arc::new(Mutex::new(broker)));
}

pub fn serve() {
	let _ = thread::spawn(move || {
		let mut broker = BROKER.get().unwrap().lock().unwrap();
		let result = broker.start();

		if let Err(e) = result {
			error!("MQTT Broker got an error: {:?}", e);
		}
	});
}
