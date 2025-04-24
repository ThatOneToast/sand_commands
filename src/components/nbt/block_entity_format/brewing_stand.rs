use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone)]
pub struct BrewingStand {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub brew_time: u32,
    pub fuel: u8,
    /// Slot 0: Left potion slot. 
    /// 
    /// Slot 1: Middle potion slot. 
    /// 
    /// Slot 2: Right potion slot. 
    /// 
    /// Slot 3: Where the potion ingredient goes. 
    /// 
    /// Slot 4: Fuel (Blaze Powder). 
    pub items: [ItemBase; 5], 
}