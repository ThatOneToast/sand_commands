use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrewingStand {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
    pub brew_time: u32,
    pub fuel: u8,
    /// Slot 0: Left potion slot. 
    /// 
    /// Slot 1: Middle potion slot. 
    /// 
    /// Slot 2: Right potion slot. 
    /// 
    /// Slot 3: Where the potion ingredient goes. 
    /// 
    /// Slot 4: Fuel (Blaze Powder). 
    pub items: [ItemBase; 5], 
}

impl ToString for BrewingStand {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize BrewingStand to JSON")
    }
}