#[derive(Clone)]
pub enum Event {
	Start,
	Stop,
	VendorStart {
		name: String,
	},

	// System events
	Time,
}
