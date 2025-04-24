use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase, LootableData};

#[derive(Debug, Clone)]
pub struct Crafter {
    pub base: BlockEntityBase,
    pub crafting_ticks_remaining: u32,
    pub triggered: bool,
    pub disabled_slots: Vec<u8>,
    pub items: [ItemBase; 9],
    pub lock: Option<String>,
    pub lootable_data: LootableData,
}