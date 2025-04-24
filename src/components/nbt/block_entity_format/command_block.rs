use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct CommandBlock {
    pub base: BlockEntityBase,
    pub auto: bool,
    pub command: String,
    pub condition_met: bool,
    pub custom_name: Option<String>,
    pub last_execution: u64,
    pub last_output: String,
    pub powered: bool,
    pub success_count: u32,
    pub track_output: bool,
    pub update_last_execution: bool,
}