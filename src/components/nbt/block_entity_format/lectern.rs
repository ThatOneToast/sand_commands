use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lectern {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book: Option<ItemBase>, // The book item without the Slot tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i8>, // Starting from 0, does not exist if there's no book
}

impl ToString for Lectern {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Lectern to JSON")
    }
}