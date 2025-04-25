use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase, LootableData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crafter {
    pub base: BlockEntityBase,
    pub crafting_ticks_remaining: u32,
    pub triggered: bool,
    pub disabled_slots: Vec<u8>,
    pub items: [ItemBase; 9],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
    pub lootable_data: LootableData,
}

impl ToString for Crafter {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Crafter to JSON")
    }
}