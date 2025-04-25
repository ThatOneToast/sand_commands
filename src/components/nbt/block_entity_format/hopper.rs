use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase, LootableData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hopper {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
    pub lootable_data: LootableData,
    pub items: [ItemBase; 9],
    pub transfer_cooldown: u32, // Time until next transfer (0 if no transfer)
}

impl ToString for Hopper {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Hopper to JSON")
    }
}