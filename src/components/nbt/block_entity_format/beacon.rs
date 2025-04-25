use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beacon {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_effect: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_effect: Option<String>,
}

impl ToString for Beacon {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Beacon to JSON")
    }
}