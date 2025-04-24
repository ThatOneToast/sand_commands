use crate::components::nbt::{block_entity_format::BlockEntityBase, entity_format::EntityData};

#[derive(Debug, Clone)]
pub struct MobSpawner {
    pub base: BlockEntityBase,
    pub spawn_data: EntityData,
    pub spawn_potentials: Vec<SpawnPotential>,
    pub min_spawn_delay: i16,
    pub max_spawn_delay: i16,
    pub spawn_count: i16,
    pub max_nearby_entities: i16,
    pub required_player_range: i16,
    pub spawn_range: i16,
    pub delay: i16,
}


#[derive(Debug, Clone)]
pub struct SpawnPotential {
    pub weight: i32,
    pub data: EntityData,
}


#[derive(Debug, Clone)]
pub struct SpawnData {
    pub entity: EntityData,
    pub custom_spawn_rules: SpawnRules,
    pub spawn_range: u8
    
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub loot_table: String,
    pub slot_drop_changes: SlotDropChance,
}

#[derive(Debug, Clone)]
pub struct SpawnRules {
    sky_light_limit: u16,
    block_light_limit: u16,
}

#[derive(Debug, Clone)]
pub struct SlotDropChance {
    /// If overall is some every value is this value
    pub overall: Option<f32>,
    pub feet: Option<f32>,
    pub legs: Option<f32>,
    pub chest: Option<f32>,
    pub head: Option<f32>,
    pub body: Option<f32>,
    pub main_hand: Option<f32>,
    pub off_hand: Option<f32>,
}
