use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct EnchantingTable {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
}