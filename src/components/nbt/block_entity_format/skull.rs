use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct Skull {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub note_block_sound: Option<String>,
    pub profile: Option<SkullProfile>,
}

#[derive(Debug, Clone)]
pub struct SkullProfile {
    pub name: Option<String>, // Player username
    pub id: Option<[i32; 4]>, // UUID as array
    pub properties: Option<Vec<SkullProperty>>,
}

#[derive(Debug, Clone)]
pub struct SkullProperty {
    pub name: String, // Can be "textures"
    pub value: String, // Base64 encoded texture data
    pub signature: Option<String>, // Base64 encoded signature
}