use crate::components::nbt::block_entity_format::{BlockEntityBase, Item, LootableData};

#[derive(Debug, Clone)]
pub struct TrappedChest {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub lootable_data: LootableData,
    pub items: Vec<Item>,
    pub gold: Option<i8>, // April fools snapshot 23w13a_or_b
}