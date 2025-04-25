use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreakingHeart {
    pub base: BlockEntityBase,
    pub creaking: [u32; 4], 
}

impl ToString for CreakingHeart {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize CreakingHeart to JSON")
    }
}