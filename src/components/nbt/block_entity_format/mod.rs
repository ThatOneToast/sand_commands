// Block Entity Format
// Based on Minecraft Wiki: https://minecraft.wiki/w/Chunk_format#Block_entity_format

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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

// BlockEntityComponents enum to store all block entity formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockEntityComponents {
    Banner(banner::Banner),
    Barrel(barrel::Barrel),
    Beacon(beacon::Beacon),
    Bed(bed::Bed),
    Beehive(beehive::Beehive),
    Bell(bell::Bell),
    BlastFurnace(blast_furnace::BlastFurnace),
    BrewingStand(brewing_stand::BrewingStand),
    CalibratedSculkSensor(calibrated_sculk_sensor::CalibratedSculkSensor),
    Campfire(campfire::Campfire),
    Chest(chest::Chest),
    ChiseledBookshelf(chiseled_bookshelf::ChiseledBookshelf),
    CommandBlock(command_block::CommandBlock),
    Comparator(comparator::Comparator),
    Conduit(conduit::Conduit),
    Crafter(crafter::Crafter),
    CreakingHeart(creaking_heart::CreakingHeart),
    DaylightDetector(daylight_detector::DaylightDetector),
    DecoratedPot(decorated_pot::DecoratedPot),
    Dispenser(dispenser::Dispenser),
    Dropper(dropper::Dropper),
    EnchantingTable(enchanting_table::EnchantingTable),
    EndGateway(end_gateway::EndGateway),
    EndPortal(end_portal::EndPortal),
    EnderChest(ender_chest::EnderChest),
    Furnace(furnace::Furnace),
    HangingSign(hanging_sign::HangingSign),
    Hopper(hopper::Hopper),
    Jigsaw(jigsaw::Jigsaw),
    Jukebox(jukebox::Jukebox),
    Lectern(lectern::Lectern),
    MobSpawner(mob_spawner::MobSpawner),
    Piston(piston::Piston),
    SculkCatalyst(sculk_catalyst::SculkCatalyst),
    SculkSensor(sculk_sensor::SculkSensor),
    ShulkerBox(shulker_box::ShulkerBox),
    Sign(sign::Sign),
    Skull(skull::Skull),
    Smoker(smoker::Smoker),
    SoulCampfire(soul_campfire::SoulCampfire),
    StructureBlock(structure_block::StructureBlock),
    SuspiciousGravel(suspicious_gravel::SuspiciousGravel),
    SuspiciousSand(suspicious_sand::SuspiciousSand),
    TrappedChest(trapped_chest::TrappedChest),
    TrialSpawner(trial_spawner::TrialSpawner),
    Vault(vault::Vault),
}

impl ToString for BlockEntityComponents {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize BlockEntityComponents to JSON")
    }
}

// Common types used across multiple block entities

// Common block entity data shared by all block entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEntityBase {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<DataComponents>>,
    /// If true, this is an invalid block entity, and this block is not immediately placed when a loaded chunk is loaded.
    /// If false, this is a normal block entity that can be immediately placed.
    pub keep_packed: bool,
}

impl ToString for BlockEntityBase {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize BlockEntityBase to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationListener {
    pub event: Event,
    pub event_delay: u32,
    pub selector: (i32, Event),
}

impl ToString for VibrationListener {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize VibrationListener to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockState {
    pub name: String, // Block resource location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>, // Block state properties
}

impl ToString for BlockState {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize BlockState to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub distance: f32,
    pub game_event: String,
    pub postion: [u32; 3],
    pub projectile_owner: [u32; 4],
    pub source: [u32; 4],
}

impl ToString for Event {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Event to JSON")
    }
}

// Shared item structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemBase {
    pub id: String,
    pub count: i8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<DataComponents>>,
}

impl ToString for ItemBase {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize ItemBase to JSON")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    // Other common item tag properties
}

impl ToString for ItemTag {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize ItemTag to JSON")
    }
}

// Shared lootable functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootableData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_table_seed: Option<i64>,
}

impl ToString for LootableData {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize LootableData to JSON")
    }
}
