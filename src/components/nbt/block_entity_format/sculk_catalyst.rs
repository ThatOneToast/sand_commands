use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SculkCatalyst {
    pub base: BlockEntityBase,
    pub cursors: Vec<SculkCharge>,
}

impl ToString for SculkCatalyst {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SculkCatalyst to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SculkCharge {
    pub charge: i32, // How many sculk charges in the cluster
    pub pos: [i32; 3], // Coordinates of the cluster
    pub decay_delay: bool, // Controls the decay of the cluster
    pub update_delay: i32, // Delay until discharge
    pub facings: Vec<i32>, // Faces where sculk veins are placed
}

impl ToString for SculkCharge {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SculkCharge to JSON")
    }
}