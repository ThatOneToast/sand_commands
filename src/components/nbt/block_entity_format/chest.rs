use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase, LootableData};

#[derive(Debug, Clone)]
pub struct Chest {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub lootable_data: LootableData,
    pub items: Vec<ItemBase>,
}
