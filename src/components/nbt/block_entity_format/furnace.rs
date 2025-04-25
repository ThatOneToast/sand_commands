use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Furnace {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
    pub lit_time_remaining: u16,
    pub cooking_time_spent: u16,
    pub cooking_total_time: u16,
    pub lit_total_time: u16,
    pub items: [ItemBase; 3],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipes_used: Option<Vec<(String, u32)>>,
}

impl ToString for Furnace {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Furnace to JSON")
    }
}

