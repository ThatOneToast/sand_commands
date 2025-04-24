use crate::components::{bees::Bees, nbt::block_entity_format::BlockEntityBase};

#[derive(Debug, Clone)]
pub struct Beehive {
    pub base: BlockEntityBase,
    pub bees: Bees,
    pub flower_pos: Option<[i32; 3]>,
}
