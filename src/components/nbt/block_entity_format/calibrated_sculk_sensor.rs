use crate::components::nbt::block_entity_format::BlockEntityBase;

use super::VibrationListener;


#[derive(Debug, Clone)]
pub struct CalibratedSculkSensor {
    pub base: BlockEntityBase,
    pub last_vibration_frequency: u32,
    pub listener: VibrationListener
}


