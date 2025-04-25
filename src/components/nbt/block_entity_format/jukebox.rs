use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jukebox {
    pub base: BlockEntityBase,
    pub record_item: ItemBase, // The item without the Slot tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks_since_song_started: Option<u32>,
}

impl ToString for Jukebox {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Jukebox to JSON")
    }
}