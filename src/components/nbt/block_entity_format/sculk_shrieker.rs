use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct SculkShrieker {
    pub base: BlockEntityBase,
    pub vibration_listener: ShriekerVibrationListener,
}

#[derive(Debug, Clone)]
pub struct ShriekerVibrationListener {
    pub event: i32,
    pub pending: Option<ShriekerPending>,
    pub selector: Option<ShriekerSelector>,
}

#[derive(Debug, Clone)]
pub struct ShriekerPending {
    pub distance: f32,
    pub source: i64,
    pub vibration: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone)]
pub struct ShriekerSelector {
    pub ticks: i32,
}