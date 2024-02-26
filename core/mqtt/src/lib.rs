use std::sync::OnceLock;

use rumqttd::{protocol::Error, Broker};
use tokio_util::sync::CancellationToken;

pub static BROKER: OnceLock<Broker> = OnceLock::new();

pub async fn serve(ct: CancellationToken) -> Result<(), Error> {
	let config = cnf::get().mqtt.clone();

	tokio::select! {
		_ = ct.cancelled() => {},
		_ = tokio::spawn(async move {
			let mut broker = Broker::new(config);
			let _ = broker.start();
		}) => {}
	}

	Ok(())
}
