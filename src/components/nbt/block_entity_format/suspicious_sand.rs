use crate::components::nbt::block_entity_format::{BlockEntityBase, Item, LootableData};

#[derive(Debug, Clone)]
pub struct SuspiciousSand {
    pub base: BlockEntityBase,
    pub lootable_data: LootableData,
    pub item: Option<Item>, // The item in the block
}