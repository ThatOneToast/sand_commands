use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct CreakingHeart {
    pub base: BlockEntityBase,
    pub creaking: [u32; 4], 
}