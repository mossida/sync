use crate::implement;

pub mod tado;
pub mod zigbee;

implement!(tado => Tado, zigbee => Zigbee);
