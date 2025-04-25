use serde::{Deserialize, Serialize};

use crate::components::nbt::{block_entity_format::BlockEntityBase, entity_format::EntityData};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl ToString for MobSpawner {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize MobSpawner to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPotential {
    pub weight: i32,
    pub data: SpawnData,
}

impl ToString for SpawnPotential {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SpawnPotential to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnData {
    pub entity: EntityData,
    pub custom_spawn_rules: SpawnRules,
    pub spawn_range: u8,
    pub equipment: Equipment,
}

impl ToString for SpawnData {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SpawnData to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub loot_table: String,
    pub slot_drop_changes: SlotDropChance,
}

impl ToString for Equipment {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Equipment to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRules {
    sky_light_limit: u16,
    block_light_limit: u16,
}

impl ToString for SpawnRules {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SpawnRules to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotDropChance {
    /// If overall is some every value is this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feet: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chest: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_hand: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_hand: Option<f32>,
}

impl ToString for SlotDropChance {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize SlotDropChance to JSON")
    }
}
