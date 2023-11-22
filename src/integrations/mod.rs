use crate::integrations::adapter::AdapterManager;
use crate::integrations::interface::InterfaceManager;

pub type Interface = dyn InterfaceManager;
pub type Adapter = dyn AdapterManager;

/** Default structures */
pub mod adapter;
pub mod interface;

/** Allowed integrations */
pub mod adapters;
