use serde::{Deserialize, Serialize};

use crate::components::{bees::Bees, nbt::block_entity_format::BlockEntityBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beehive {
    pub base: BlockEntityBase,
    pub bees: Bees,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flower_pos: Option<[i32; 3]>,
}

impl ToString for Beehive {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Beehive to JSON")
    }
}
