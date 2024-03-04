use std::sync::OnceLock;

use bus::Bus;

pub use event::*;

mod bus;
mod event;

pub static BUS: OnceLock<Bus<Event>> = OnceLock::new();

pub fn init() {
	let _ = BUS.set(Bus::new());
}

pub fn get() -> &'static Bus<Event> {
	BUS.get().unwrap()
}
