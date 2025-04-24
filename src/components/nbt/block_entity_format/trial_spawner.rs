use crate::components::nbt::block_entity_format::BlockEntityBase;

#[derive(Debug, Clone)]
pub struct TrialSpawner {
    pub base: BlockEntityBase,
    pub required_player_range: i32, // Maximum distance for players to join battle
    pub target_cooldown_length: i32, // Time in ticks of cooldown period
    pub normal_config: TrialSpawnerConfig, // Config when not ominous
    pub ominous_config: Option<TrialSpawnerConfig>, // Config when ominous
    pub registered_players: Vec<[i32; 4]>, // Players that joined the battle (UUID as array)
    pub current_mobs: Vec<[i32; 4]>, // Mobs spawned by this spawner and still alive (UUID as array)
    pub cooldown_ends_at: i64, // Gametime when cooldown ends
    pub next_mob_spawns_at: i64, // Gametime when next spawn attempt happens
    pub total_mobs_spawned: i32, // Total amount of mobs already spawned
    pub spawn_data: SpawnData, // Next mob to attempt to spawn
    pub ejecting_loot_table: Option<String>, // Loot table for reward
}

#[derive(Debug, Clone)]
pub struct TrialSpawnerConfig {
    // Configuration fields
    pub id: Option<String>,
    // Other configuration fields would be defined based on the complete structure
}

#[derive(Debug, Clone)]
pub struct SpawnData {
    // Spawn data fields
    pub entity_type: String,
    // Other spawn data fields would be defined here
}