use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Smoker {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
    pub lit_time_remaining: i16,
    pub cooking_time_spent: i16,
    pub cooking_total_time: i16,
    pub lit_total_time: i16,
    pub items: Vec<ItemBase>,
    pub recipes_used: Vec<(String, i32)>,
}

impl ToString for Smoker {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Smoker to JSON")
    }
}

