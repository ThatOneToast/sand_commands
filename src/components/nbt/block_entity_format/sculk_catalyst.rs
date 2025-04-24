use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct SculkCatalyst {
    pub base: BlockEntityBase,
    pub cursors: Vec<SculkCharge>,
}

#[derive(Debug, Clone)]
pub struct SculkCharge {
    pub charge: i32, // How many sculk charges in the cluster
    pub pos: [i32; 3], // Coordinates of the cluster
    pub decay_delay: i32, // Controls the decay of the cluster
    pub update_delay: i32, // Delay until discharge
    pub facings: Vec<i32>, // Faces where sculk veins are placed
}