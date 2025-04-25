use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HangingSign {
    pub base: BlockEntityBase,
    pub is_waxed: bool,
    pub front_text: SignText,
    pub back_text: SignText,
}

impl ToString for HangingSign {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize HangingSign to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignText {
    pub has_glowing_text: bool,
    pub color: String, // Default is "black"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_messages: Option<Vec<String>>, // Only in Realms
    pub messages: Vec<String>, // Raw JSON text format
}

impl ToString for SignText {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SignText to JSON")
    }
}