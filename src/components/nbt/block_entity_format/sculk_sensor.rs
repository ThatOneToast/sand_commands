use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SculkSensor {
    pub base: BlockEntityBase,
    pub last_vibration_frequency: i32,
    pub listener: VibrationListener,
}

impl ToString for SculkSensor {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SculkSensor to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationListener {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<VibrationEvent>,
    pub event_delay: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<VibrationEvent>,
    pub tick: i64,
}

impl ToString for VibrationListener {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize VibrationListener to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationEvent {
    pub distance: f32,
    pub game_event: String,
    pub pos: [f64; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projectile_owner: Option<[i32; 4]>, // UUID as array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<[i32; 4]>, // UUID as array
}

impl ToString for VibrationEvent {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize VibrationEvent to JSON")
    }
}
