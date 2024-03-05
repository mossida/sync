mod service;

pub struct Engine {}

pub fn init() {
	let _ = bus::get();
}
