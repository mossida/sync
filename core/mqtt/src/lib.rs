use std::sync::OnceLock;

use rumqttd::{protocol::Error, Broker};
use tokio_util::sync::CancellationToken;

pub static BROKER: OnceLock<Broker> = OnceLock::new();

pub async fn serve(ct: CancellationToken) -> Result<(), Error> {
	loop {
		tokio::select! {
			//
			biased;
			_ = ct.cancelled() => break,
			_ = tokio::spawn(async move {
				let mut broker = Broker::new(cnf::get().mqtt.clone());
				broker.start().unwrap();
			}) => {}
		}
	}

	Ok(())
}
