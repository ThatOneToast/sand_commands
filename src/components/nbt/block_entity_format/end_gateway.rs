use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndGateway {
    pub base: BlockEntityBase,
    pub age: u32,
    pub exact_teleport: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_portal: Option<[i32; 3]>, // Location entities are teleported to
}

impl ToString for EndGateway {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize EndGateway to JSON")
    }
}