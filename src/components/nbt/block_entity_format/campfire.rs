use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campfire {
    pub base: BlockEntityBase,
    pub cooking_times: [u32; 4],
    pub cooking_total_times: [u32; 4],
    pub items: [ItemBase; 4],
}

impl ToString for Campfire {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Campfire to JSON")
    }
}