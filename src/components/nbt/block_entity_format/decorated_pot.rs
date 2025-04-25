use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase, LootableData};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoratedPot {
    pub base: BlockEntityBase,
    pub sherds: [String; 4], 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<ItemBase>,
    pub lootable_data: LootableData,
}

impl ToString for DecoratedPot {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize DecoratedPot to JSON")
    }
}