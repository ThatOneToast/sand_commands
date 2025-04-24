use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct Comparator {
    pub base: BlockEntityBase,
    pub output_signal: i32,
}