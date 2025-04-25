use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conduit {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<[u32; 4]>, // UUID as array
}

impl ToString for Conduit {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Conduit to JSON")
    }
}