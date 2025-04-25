use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnchantingTable {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
}

impl ToString for EnchantingTable {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize EnchantingTable to JSON")
    }
}