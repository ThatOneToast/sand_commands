use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparator {
    pub base: BlockEntityBase,
    pub output_signal: i32,
}

impl ToString for Comparator {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Comparator to JSON")
    }
}