use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct Jigsaw {
    pub base: BlockEntityBase,
    pub final_state: String, // The block this jigsaw becomes
    pub joint: String, // "rollable" or "aligned"
    pub name: String, // Jigsaw block's name
    pub pool: String, // Target pool to select a structure from
    pub target: String, // Target name
    pub selection_priority: i32, // Priority for generation selection
    pub placement_priority: i32, // Priority for placing children
}