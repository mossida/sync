pub trait IntoBridgeMethod {
	fn into_bridge_method(&self) -> Result<BridgeMethod, ()>;
}
