use std::sync::OnceLock;

pub use bus::Bus;

pub use event::*;

mod bus;
mod consumer;
mod event;

pub use consumer::Consumer;

pub static BUS: OnceLock<Bus<Event>> = OnceLock::new();

pub fn init() {
	let bus = Bus::new();
	let _ = BUS.set(bus);
}

pub fn get() -> &'static Bus<Event> {
	BUS.get().unwrap()
}
