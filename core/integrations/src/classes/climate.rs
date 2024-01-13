use derive_more::Display;
use serde::{Deserialize, Serialize};
use uom::si::f64::{Ratio, TemperatureInterval, ThermodynamicTemperature};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Swing {
	On,
	Off,
	Vertical,
	Horizontal,
	Both,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Preset {
	Home,
	Away,
	Sleep,
	Manual,
	Off,
	Boost,
	Eco,
	Comfort,
	Unknown,
}

#[derive(Debug, Serialize, Deserialize, Display, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HVACMode {
	Off,
	Heat,
	Cool,
	Auto,
	Fan,
	Dry,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum HVACAction {
	Preheating,
	Idle,
	Heating,
	Cooling,
	Drying,
	Fan,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FanStatus {
	On,
	Off,
	Auto,
	Low,
	Medium,
	High,
	Top,
	Middle,
	Focus,
	Diffuse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Attribute {
	AuxHeat(bool),
	CurrentTemperature(ThermodynamicTemperature),
	CurrentHumidity(Ratio),
	Preset(Preset),
	FanModes(Vec<FanStatus>),
	FanMode(FanStatus),
	HvacModes(Vec<HVACMode>),
	HvacMode(HVACMode),
	HvacAction(HVACAction),
	HvacPower(Ratio),
	Humidity(Ratio),
	MaxHumidity(Ratio),
	MinHumidity(Ratio),
	MaxTemp(ThermodynamicTemperature),
	MinTemp(ThermodynamicTemperature),
	TargetTemperatureHigh(ThermodynamicTemperature),
	TargetTemperatureLow(ThermodynamicTemperature),
	TargetTemperatureStep(TemperatureInterval),
}
