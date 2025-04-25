use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::BlockEntityBase;

use super::mob_spawner::{SpawnData, SpawnPotential};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialSpawner {
    pub base: BlockEntityBase,
    pub required_player_range: i32, // Maximum distance for players to join battle
    pub target_cooldown_length: i32, // Time in ticks of cooldown period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_config: Option<TrialSpawnerConfig>, // Config when not ominous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ominous_config: Option<TrialSpawnerConfig>, // Config when ominous
    pub registered_players: Vec<[u32; 4]>,
    pub current_mobs: Vec<[u32; 4]>,
    pub cooldown_ends_at: u32,
    pub next_mob_spawns_at: u32,
    pub total_mobs_spawned: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawn_data: Option<SpawnData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ejecting_loot_table: Option<String>
    
}

impl ToString for TrialSpawner {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize TrialSpawner to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialSpawnerConfig {
    pub spawn_range: u8,
    pub total_mobs: u8,
    pub simultaneous_mobs: u8,
    pub total_mobs_added_per_player: u8,
    pub simultaneous_mobs_added_per_player: u8,
    pub ticks_between_spawn: u16,
    pub spawn_potionetials: Vec<SpawnPotential>,
    pub loot_tables_to_eject: Vec<(i32, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_to_drop_when_ominous: Option<String>
}

impl ToString for TrialSpawnerConfig {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize TrialSpawnerConfig to JSON")
    }
}
