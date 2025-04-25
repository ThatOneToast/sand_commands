use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBlock {
    pub base: BlockEntityBase,
    pub auto: bool,
    pub command: String,
    pub condition_met: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    pub last_execution: u64,
    pub last_output: String,
    pub powered: bool,
    pub success_count: u32,
    pub track_output: bool,
    pub update_last_execution: bool,
}

impl ToString for CommandBlock {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize CommandBlock to JSON")
    }
}