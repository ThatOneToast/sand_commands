use serde::{Deserialize, Serialize};

use crate::components::nbt::block_entity_format::{BlockEntityBase, ItemBase};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub base: BlockEntityBase,
    pub config: VaultConfig,
    pub server_data: VaultServerData,
    pub shared_data: VaultSharedData,
}

impl ToString for Vault {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Vault to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_loot_table_to_display: Option<String>,
    pub activation_range: f32,
    pub deactivation_range: f32,
    pub key_item: ItemBase,
}

impl ToString for VaultConfig {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize VaultConfig to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultServerData {
    pub rewarded_players: Vec<[u32; 4]>, // UUIDs as arrays
    pub state_updating_resumes_at: u32,
    pub items_to_eject: Vec<ItemBase>,
    pub total_ejections_needed: u32,
}

impl ToString for VaultServerData {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize VaultServerData to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSharedData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_item: Option<ItemBase>,
    pub connected_players: Vec<[u32; 4]>, // UUIDs as arrays
    pub connected_particles_range: f32,
}

impl ToString for VaultSharedData {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize VaultSharedData to JSON")
    }
}