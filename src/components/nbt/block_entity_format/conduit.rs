use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct Conduit {
    pub base: BlockEntityBase,
    pub target: Option<[u32; 4]>, // UUID as array
}