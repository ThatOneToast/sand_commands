use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulCampfire {
    pub base: BlockEntityBase,
    pub cooking_times: [i32; 4],
    pub cooking_total_times: [i32; 4],
    pub items: Vec<ItemBase>,
}

impl ToString for SoulCampfire {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SoulCampfire to JSON")
    }
}