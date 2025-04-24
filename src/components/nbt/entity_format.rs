use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityData {
    /// How much air the entity has, in game ticks.
    /// Decreases when unable to breathe (except suffocating in a block).
    /// Increases when it can breathe.
    /// [Short] Air being <= -20 game ticks (while still unable to breathe) on a given game tick causes the entity to immediately lose 1 health to drowning damage.
    /// This resets [Short] Air to 0 game ticks.
    /// Most mobs can have a maximum of 300 game ticks (15 seconds) of [Short] Air, while dolphins can reach up to 4800 game ticks (240 seconds), and axolotls have 6000 game ticks (300 seconds).
    /// None = Default for the given entity type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air: Option<i16>,
    /// The custom name JSON text component of this entity.
    /// Appears in player death messages and villager trading interfaces, as well as above the entity when the player's cursor is over it.
    /// May be empty or not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// If true, and this entity has a custom name, the name always appears above the entity, regardless of where the cursor points.
    /// If the entity does not have a custom name, a default name is shown. May not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name_visible: Option<bool>,
    /// Any custom data. Key-Value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CustomData>,
    /// Distance the entity has fallen. Larger values cause more damage when the entity lands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fall_distance: Option<f32>,
    /// Number of game ticks until the fire is put out.
    /// Negative values reflect how long the entity can stand in fire before burning.
    /// Default to -20 when not on fire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire: Option<i32>,
    /// If true, the entity has a glowing outline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glowing: Option<bool>,
    /// If true, the entity visually appears on fire, even if it is not actually on fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visual_fire: Option<bool>,
    /// Does not exist for the Player entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If true, the entity should not take damage.
    /// This applies to living and nonliving entities alike: mobs should not take damage from any source (including potion effects),
    /// and cannot be moved by fishing rods, attacks, explosions, or projectiles, and objects such as vehicles and item frames cannot be destroyed unless their supports are removed.
    /// Invulnerable player entities are also ignored by any hostile mobs.
    /// Note that these entities can be damaged by players in Creative mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invulnerable: Option<bool>,
    /// List of 3 [Double] doubles describing the current dX, dY, and dZ velocity of the entity in meters per game tick.
    /// Only allows between 10.0 and -10.0 (inclusive), else resets to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion: Option<[f32; 3]>,
    /// If true, the entity does not fall down naturally. Set to true by striders in lava.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_gravity: Option<bool>,
    /// If true, the entity is touching the ground
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_ground: Option<bool>,
    /// The data of the entities that are riding this entity.
    /// The same as this format (recursive).
    /// Note that each passenger entity, and the ridden entity (the vehicle) have equal control of the movement of the ridden entity.
    /// The topmost entity controls spawning conditions when the vehicle entity is created by a mob spawner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passengers: Option<Vec<EntityData>>,
    /// The number of game ticks before which the entity may be teleported back through a nether portal.
    /// Initially starts at 300 game ticks (15 seconds) after teleportation and counts down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_cooldown: Option<i32>,
    /// List of 3 [Double] doubles describing the current X, Y, and Z position (coordinates) of the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<[f32; 3]>,
    /// List of 2 floats representing the rotation of the entity's facing direction, in degrees.
    /// Facing direction can also be described as a looking direction, for most entity's that have heads.
    /// 0: The yaw of the entity's orientation. Yaw is the rotation around the Y axis (called yaw).
    /// Values vary from -180 degrees to +180 degrees, rather than from 0 to 360. As the entity turns to the right, this value goes up, and as the entity turns left, this value does down.
    /// See table of specific values here: Chunk format/Entity/Rotation (yaw).
    /// 1: The pitch of the entity's orientation. Pitch is the offset from the horizon.
    /// Pitch = 0 means the direction is horizontal. A positive pitch (pitch > 0) means the entity is facing downward to some degree, or that the facing direction is facing below the horizon (toward the ground).
    /// A negative pitch (pitch > 0) means the entity is facing above the horizon (toward higher ground of the sky).
    /// Pitch is always between -90 and +90 degrees, where pitch = -90 means facing directly down, and pitch = +90 means facing directly up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f32; 2]>,
    /// If true, this entity is silenced. May not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// List of scoreboard tags of this entity. It is not preserved if it is removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Optional. How many game ticks the entity has been freezing.
    /// Although this tag is defined for all entities, it is actually only used by mobs that are not in the freeze_immune_entity_types entity type tag.
    /// Increases while in powder snow, even partially, up to a maximum of 300 game ticks (15 seconds), and decreases at double speed while not in powder snow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticks_freezing: Option<i32>,
    /// This entity's Universally Unique IDentifier. The 128-bit UUID is stored as four 32-bit integers ([Int] Ints), ordered from most to least significant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<[i32; 4]>,
}

impl ToString for EntityData {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("failed to serialize entity data into json")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomData(pub HashMap<String, String>);

impl ToString for CustomData {
    fn to_string(&self) -> String {
        let data = self
            .0
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<String>>()
            .join(",");
        format!("custom_data={{{data}}}")
    }
}
