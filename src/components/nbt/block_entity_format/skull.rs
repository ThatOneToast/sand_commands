use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skull {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_block_sound: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<SkullProfile>,
}

impl ToString for Skull {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Skull to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkullProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>, // Player username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<[i32; 4]>, // UUID as array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SkullProperty>>,
}

impl ToString for SkullProfile {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SkullProfile to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkullProperty {
    pub name: String, // Can be "textures"
    pub value: String, // Base64 encoded texture data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>, // Base64 encoded signature
}

impl ToString for SkullProperty {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SkullProperty to JSON")
    }
}