use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone)]
pub struct Jukebox {
    pub base: BlockEntityBase,
    pub record_item: ItemBase, // The item without the Slot tag
    pub ticks_since_song_started: Option<u32>,
}