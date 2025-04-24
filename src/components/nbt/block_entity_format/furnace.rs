use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone)]
pub struct Furnace {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub lit_time_remaining: u16,
    pub cooking_time_spent: u16,
    pub cooking_total_time: u16,
    pub lit_total_time: u16,
    pub items: [ItemBase; 3],
    pub recipes_used: Option<Vec<(String, u32)>>,
}

