use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaylightDetector {
    pub base: BlockEntityBase,
}

impl ToString for DaylightDetector {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize DaylightDetector to JSON")
    }
}