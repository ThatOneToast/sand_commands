use serde::{Deserialize, Serialize};

use crate::components::nbt::{block_entity_format::{BlockEntityBase, BlockState}, Looking};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Piston {
    pub base: BlockEntityBase,
    pub block_state: BlockState,
    pub extending: bool,
    pub facing: Looking, // 0=down, 1=up, 2=north, 3=south, 4=west, 5=east
    pub progress: f32,
    pub source: bool, // true if the block represents the piston head, false if it's a pushed block
}

impl ToString for Piston {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Piston to JSON")
    }
}
