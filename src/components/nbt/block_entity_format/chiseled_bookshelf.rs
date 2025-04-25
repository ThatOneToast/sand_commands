use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChiseledBookshelf {
    pub base: BlockEntityBase,
    pub items: [Option<ItemBase>; 5],
    pub last_interacted_slot: i8, 
}

impl ToString for ChiseledBookshelf {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize ChiseledBookshelf to JSON")
    }
}