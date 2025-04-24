use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct SculkSensor {
    pub base: BlockEntityBase,
    pub last_vibration_frequency: i32,
    pub listener: VibrationListener,
}

#[derive(Debug, Clone)]
pub struct VibrationListener {
    pub event: Option<VibrationEvent>,
    pub event_delay: i32,
    pub selector: Option<VibrationSelector>,
    pub tick: i64,
}

#[derive(Debug, Clone)]
pub struct VibrationEvent {
    pub distance: f32,
    pub game_event: String,
    pub pos: [f64; 3],
    pub projectile_owner: Option<[i32; 4]>, // UUID as array
    pub source: Option<[i32; 4]>, // UUID as array
}

#[derive(Debug, Clone)]
pub struct VibrationSelector {
    // Vibration selector data
    pub event: Option<VibrationEvent>,
}