use crate::components::nbt::block_entity_format::{BlockEntityBase, Item};

#[derive(Debug, Clone)]
pub struct Smoker {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub lit_time_remaining: i16,
    pub cooking_time_spent: i16,
    pub cooking_total_time: i16,
    pub lit_total_time: i16,
    pub items: Vec<Item>,
    pub recipes_used: Option<RecipesUsed>,
}

#[derive(Debug, Clone)]
pub struct RecipesUsed {
    pub recipes: Vec<(String, i32)>, // Recipe ID -> count map
}