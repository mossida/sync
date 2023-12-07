use async_trait::async_trait;
use ractor::Actor;
use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use surrealdb::sql::Id;

use crate::devices::models::DeviceId;
use crate::entities::models::{Entity, EntityAttributes, EntityFactory, EntityId};
use crate::integrations::classes::Class;
use crate::states::models::state::StateFactory;

#[async_trait]
pub trait Climate: Actor {}

impl<T> EntityFactory for T
where
    T: Climate,
{
    fn build_entity(device_id: DeviceId) -> Entity {
        Entity {
            id: EntityId::new(Id::rand().to_string()).unwrap(),
            enabled: true,
            available: true,
            class: Class::Climate,
            attributes: EntityAttributes {},
            device: device_id,
        }
    }
}

impl<T> StateFactory for T
where
    T: Climate,
{
    type State = Preset;
    type Attributes = Attributes;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Swing {
    On,
    Off,
    Vertical,
    Horizontal,
    Both,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HVACMode {
    Off,
    Heat,
    Cool,
    Auto,
    Fan,
    Dry,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HVACAction {
    Preheating,
    Idle,
    Heating,
    Cooling,
    Drying,
    Fan,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum Attributes {
    AuxHeat(bool),
    CurrentTemperature(f32),
    CurrentHumidity(f32),
    FanModes(Vec<FanStatus>),
    FanMode(FanStatus),
    HVACModes(Vec<HVACMode>),
    HVACMode(HVACMode),
    HVACAction(HVACAction),
    Humidity(f32),
    MaxHumidity(f32),
    MinHumidity(f32),
    MaxTemp(f32),
    MinTemp(f32),
    TargetTemperatureHigh(f32),
    TargetTemperatureLow(f32),
    TargetTemperatureStep(f32),
}
