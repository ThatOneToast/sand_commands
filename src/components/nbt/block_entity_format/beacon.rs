use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct Beacon {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub primary_effect: Option<String>,
    pub secondary_effect: Option<String>,
}