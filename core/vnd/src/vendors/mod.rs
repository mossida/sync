use crate::implement;

pub mod tado;
pub mod zigbee;

pub mod any;

implement!(tado => Tado, zigbee => Zigbee);
