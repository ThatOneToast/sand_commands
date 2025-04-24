use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase, LootableData};


#[derive(Debug, Clone)]
pub struct DecoratedPot {
    pub base: BlockEntityBase,
    pub sherds: [String; 4], 
    pub item: Option<ItemBase>,
    pub lootable_data: LootableData,
}