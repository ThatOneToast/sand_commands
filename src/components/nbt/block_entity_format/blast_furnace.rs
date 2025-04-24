use crate::components::nbt::block_entity_format::BlockEntityBase;

use super::ItemBase;

#[derive(Debug, Clone)]
pub struct BlastFurnace {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub lit_time_remaining: i16,
    pub cooking_time_spent: i16,
    pub cooking_total_time: i16,
    pub lit_total_time: i16,
    pub items: [ItemBase; 3],
    pub recipes_used: Vec<(String, i32)>,
}
