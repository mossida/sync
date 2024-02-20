use std::sync::OnceLock;

use bus::Bus;

pub use event::*;

mod bus;
mod event;

static BUS: OnceLock<Bus> = OnceLock::new();

pub async fn init() {
	let _ = BUS.set(Bus::new());
}
