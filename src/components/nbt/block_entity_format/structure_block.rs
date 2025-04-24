use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct StructureBlock {
    pub base: BlockEntityBase,
    pub author: String,
    pub ignore_entities: bool,
    pub integrity: f32,
    pub metadata: String,
    pub mirror: String, // "NONE", "LEFT_RIGHT", or "FRONT_BACK"
    pub mode: String, // "SAVE", "LOAD", "CORNER", or "DATA"
    pub name: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub powered: bool,
    pub rotation: String, // "NONE", "CLOCKWISE_90", "CLOCKWISE_180", or "COUNTERCLOCKWISE_90"
    pub seed: i64,
    pub show_bounding_box: bool,
    pub size_x: i32,
    pub size_y: i32,
    pub size_z: i32,
}