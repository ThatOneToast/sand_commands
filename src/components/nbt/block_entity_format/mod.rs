// Block Entity Format
// Based on Minecraft Wiki: https://minecraft.wiki/w/Chunk_format#Block_entity_format

use std::collections::HashMap;

use crate::components::DataComponents;

pub mod banner;
pub mod barrel;
pub mod beacon;
pub mod bed;
pub mod beehive;
pub mod bell;
pub mod blast_furnace;
pub mod brewing_stand;
pub mod calibrated_sculk_sensor;
pub mod campfire;
pub mod chest;
pub mod chiseled_bookshelf;
pub mod command_block;
pub mod comparator;
pub mod conduit;
pub mod crafter;
pub mod creaking_heart;
pub mod daylight_detector;
pub mod decorated_pot;
pub mod dispenser;
pub mod dropper;
pub mod enchanting_table;
pub mod end_gateway;
pub mod end_portal;
pub mod ender_chest;
pub mod furnace;
pub mod hanging_sign;
pub mod hopper;
pub mod jigsaw;
pub mod jukebox;
pub mod lectern;
pub mod mob_spawner;
pub mod piston;
pub mod sculk_catalyst;
pub mod sculk_sensor;
pub mod sculk_shrieker;
pub mod shulker_box;
pub mod sign;
pub mod skull;
pub mod smoker;
pub mod soul_campfire;
pub mod structure_block;
pub mod suspicious_gravel;
pub mod suspicious_sand;
pub mod trapped_chest;
pub mod trial_spawner;
pub mod vault;

// Common types used across multiple block entities

// Common block entity data shared by all block entities
#[derive(Debug, Clone)]
pub struct BlockEntityBase {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub components: Option<Vec<DataComponents>>,
    /// If true, this is an invalid block entity, and this block is not immediately placed when a loaded chunk is loaded.
    /// If false, this is a normal block entity that can be immediately placed.
    pub keep_packed: bool,
}

#[derive(Debug, Clone)]
pub struct VibrationListener {
    pub event: Event,
    pub event_delay: u32,
    pub selector: (i32, Event),
}

#[derive(Debug, Clone)]
pub struct BlockState {
    pub name: String, // Block resource location
    pub properties: Option<HashMap<String, String>>, // Block state properties
}

#[derive(Debug, Clone)]
pub struct Event {
    pub distance: f32,
    pub game_event: String,
    pub postion: [u32; 3],
    pub projectile_owner: [u32; 4],
    pub source: [u32; 4],
}

// Shared item structure
#[derive(Debug, Clone)]
pub struct ItemBase {
    pub id: String,
    pub count: i8,
    pub slot: Option<i8>,
    pub components: Option<Vec<DataComponents>>,
}

#[derive(Debug, Clone)]
pub struct ItemTag {
    pub damage: Option<i32>,
    pub custom_name: Option<String>,
    // Other common item tag properties
}

// Shared lootable functionality
#[derive(Debug, Clone)]
pub struct LootableData {
    pub loot_table: Option<String>,
    pub loot_table_seed: Option<i64>,
}
