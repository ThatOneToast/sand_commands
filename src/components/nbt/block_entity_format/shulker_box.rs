use crate::components::nbt::block_entity_format::{BlockEntityBase, Item, LootableData};

#[derive(Debug, Clone)]
pub struct ShulkerBox {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub lootable_data: LootableData,
    pub items: Vec<Item>,
}