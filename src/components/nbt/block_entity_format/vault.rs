use crate::components::nbt::block_entity_format::{BlockEntityBase, Item};

#[derive(Debug, Clone)]
pub struct Vault {
    pub base: BlockEntityBase,
    pub config: VaultConfig,
    pub server_data: VaultServerData,
    pub shared_data: VaultSharedData,
}

#[derive(Debug, Clone)]
pub struct VaultConfig {
    pub loot_table: Option<String>,
    pub override_loot_table_to_display: Option<String>,
    pub activation_range: f64,
    pub deactivation_range: f64,
    pub key_item: Item,
}

#[derive(Debug, Clone)]
pub struct VaultServerData {
    pub rewarded_players: Vec<[i32; 4]>, // UUIDs as arrays
    pub state_updating_resumes_at: i64,
    pub items_to_eject: Vec<Item>,
    pub total_ejections_needed: i32,
}

#[derive(Debug, Clone)]
pub struct VaultSharedData {
    pub display_item: Option<Item>,
    pub connected_players: Vec<[i32; 4]>, // UUIDs as arrays
    pub connected_particles_range: f64,
}