use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bed {
    pub base: BlockEntityBase,
}

impl ToString for Bed {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Bed to JSON")
    }
}