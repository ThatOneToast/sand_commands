use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone)]
pub struct Campfire {
    pub base: BlockEntityBase,
    pub cooking_times: [u32; 4],
    pub cooking_total_times: [u32; 4],
    pub items: [ItemBase; 4],
}