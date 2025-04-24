use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct EndGateway {
    pub base: BlockEntityBase,
    pub age: u32,
    pub exact_teleport: bool,
    pub exit_portal: Option<[i32; 3]>, // Location entities are teleported to
}