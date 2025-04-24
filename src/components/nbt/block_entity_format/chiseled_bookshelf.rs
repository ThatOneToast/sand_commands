use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};


#[derive(Debug, Clone)]
pub struct ChiseledBookshelf {
    pub base: BlockEntityBase,
    pub items: [Option<ItemBase>; 5],
    pub last_interacted_slot: i8, 
}