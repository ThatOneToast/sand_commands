use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

use super::VibrationListener;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibratedSculkSensor {
    pub base: BlockEntityBase,
    pub last_vibration_frequency: u32,
    pub listener: VibrationListener
}

impl ToString for CalibratedSculkSensor {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize CalibratedSculkSensor to JSON")
    }
}

