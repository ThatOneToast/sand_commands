use crate::components::nbt::block_entity_format::{BlockEntityBase, Item};

#[derive(Debug, Clone)]
pub struct SoulCampfire {
    pub base: BlockEntityBase,
    pub cooking_times: [i32; 4],
    pub cooking_total_times: [i32; 4],
    pub items: Vec<Item>,
}