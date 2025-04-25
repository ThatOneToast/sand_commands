use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureBlockMode {
    Save,
    Load,
    Corner,
    Data,
}

impl ToString for StructureBlockMode {
    fn to_string(&self) -> String {
        match self {
            StructureBlockMode::Save => "SAVE".to_string(),
            StructureBlockMode::Load => "LOAD".to_string(),
            StructureBlockMode::Corner => "CORNER".to_string(),
            StructureBlockMode::Data => "DATA".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureBlockMirror {
    None,
    LeftRight,
    FrontBack,
}

impl ToString for StructureBlockMirror {
    fn to_string(&self) -> String {
        match self {
            StructureBlockMirror::None => "NONE".to_string(),
            StructureBlockMirror::LeftRight => "LEFT_RIGHT".to_string(),
            StructureBlockMirror::FrontBack => "FRONT_BACK".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureBlockRotation {
    None,
    Clockwise90,
    Clockwise180,
    Counterclockwise90,
}

impl ToString for StructureBlockRotation {
    fn to_string(&self) -> String {
        match self {
            StructureBlockRotation::None => "NONE".to_string(),
            StructureBlockRotation::Clockwise90 => "CLOCKWISE_90".to_string(),
            StructureBlockRotation::Clockwise180 => "CLOCKWISE_180".to_string(),
            StructureBlockRotation::Counterclockwise90 => "COUNTERCLOCKWISE_90".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureBlock {
    pub base: BlockEntityBase,
    pub author: String,
    pub ignore_entities: bool,
    pub integrity: f32,
    pub metadata: String,
    pub mirror: StructureBlockMirror,
    pub mode: StructureBlockMode,
    pub name: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub powered: bool,
    pub rotation: StructureBlockRotation,
    pub seed: i64,
    pub show_bounding_box: bool,
    pub size_x: i32,
    pub size_y: i32,
    pub size_z: i32,
}

impl ToString for StructureBlock {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize StructureBlock to JSON")
    }
}