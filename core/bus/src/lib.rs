use std::{sync::OnceLock, time::Duration};

use bus::Bus;

pub use event::*;
use futures::StreamExt;

use tokio_stream::wrappers::IntervalStream;

mod bus;
mod consumer;
mod event;

pub use consumer::Consumer;

pub static BUS: OnceLock<Bus<Event>> = OnceLock::new();

fn system(bus: &Bus<Event>) {
	// Time tick
	let interval = tokio::time::interval(Duration::from_secs(1));
	let stream = IntervalStream::new(interval).map(|_| Event::Time);

	bus.publish(stream);
}

pub fn init() {
	let bus = Bus::new();

	// Init all the system events
	system(&bus);

	let _ = BUS.set(bus);
}

pub fn get() -> &'static Bus<Event> {
	BUS.get().unwrap()
}
