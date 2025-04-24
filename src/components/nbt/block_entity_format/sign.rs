use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct Sign {
    pub base: BlockEntityBase,
    pub is_waxed: bool,
    pub front_text: SignText,
    pub back_text: SignText,
}

#[derive(Debug, Clone)]
pub struct SignText {
    pub has_glowing_text: bool,
    pub color: String, // Default is "black"
    pub filtered_messages: Option<Vec<String>>, // Only in Realms
    pub messages: Vec<String>, // Raw JSON text format
}