use std::{sync::OnceLock, thread};

use rumqttd::{
	local::{self, LinkRx, LinkTx},
	Broker,
};

pub use rumqttd;
use tokio::sync::Mutex;

pub static BROKER: OnceLock<Mutex<Broker>> = OnceLock::new();

pub fn init() {
	let config = cnf::get().mqtt.clone();
	let broker = Broker::new(config);

	let _ = BROKER.set(Mutex::new(broker));
}

pub async fn link(id: &str) -> Result<(LinkTx, LinkRx), local::LinkError> {
	// TODO: handle the error
	let router = BROKER.get().unwrap().lock().await;
	router.link(id)
}

pub async fn serve() {
	// TODO: handle the error
	let mut router = BROKER.get().unwrap().lock().await;

	let _ = thread::spawn(move || {
		let _ = router.start();
	});
}
