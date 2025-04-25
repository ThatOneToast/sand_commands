use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase, LootableData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousSand {
    pub base: BlockEntityBase,
    pub lootable_data: LootableData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<ItemBase>, // The item in the block
}

impl ToString for SuspiciousSand {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SuspiciousSand to JSON")
    }
}