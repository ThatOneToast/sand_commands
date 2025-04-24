use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone)]
pub struct Lectern {
    pub base: BlockEntityBase,
    pub book: Option<ItemBase>, // The book item without the Slot tag
    pub page: Option<i8>, // Starting from 0, does not exist if there's no book
}