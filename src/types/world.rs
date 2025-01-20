pub type PlayerName = String;
pub type TickDuration = String;

pub trait MinecraftDimension: ToString + std::fmt::Debug {
    fn namespace(&self) -> String;
    fn name(&self) -> String;
}


#[derive(Debug, Clone)]
pub enum Yaw {
    NORTH,
    NORTHEAST,
    NORTHWEST,
    SOUTH,
    SOUTHEAST,
    SOUTHWEST,
    EAST,
    WEST,
    Custom(f32),
}

impl ToString for Yaw {
    fn to_string(&self) -> String {
        match self {
            Self::NORTH => "-180.0".to_string(),
            Self::EAST => "-90.0".to_string(),
            Self::WEST => "90.0".to_string(),
            Self::SOUTH => "180.0".to_string(),
            Self::NORTHEAST => "45.0".to_string(), 
            Self::NORTHWEST => "-45.0".to_string(),
            Self::SOUTHEAST => "225.0".to_string(),
            Self::SOUTHWEST => "-135.0".to_string(), 
            Self::Custom(value) => format!("{value}"),
        }
    }
}
            
#[derive(Debug, Clone)]
pub enum MinecraftDimensions {
    Overworld,
    Nether,
    End,
}

#[derive(Debug, Clone)]
pub enum HeightMap {
    WorldSurface,
    MotionBlocking,
    MotionBlockingNoLeaves,
    OceanFloor,
}

impl ToString for HeightMap {
    fn to_string(&self) -> String {
        match self {
            HeightMap::WorldSurface => "world_surface".to_string(),
            HeightMap::MotionBlocking => "motion_blocking".to_string(),
            HeightMap::MotionBlockingNoLeaves => "motion_blocking_no_leaves".to_string(),
            HeightMap::OceanFloor => "ocean_floor".to_string(),
        }
    }
}

impl ToString for MinecraftDimensions {
    fn to_string(&self) -> String {
        format!("{}:{}", self.namespace(), self.name())
    }
}

impl MinecraftDimension for MinecraftDimensions {
    fn namespace(&self) -> String {
        "minecraft".to_string()
    }
    
    fn name(&self) -> String {
        match self {
            MinecraftDimensions::Overworld => "overworld".to_string(),
            MinecraftDimensions::Nether => "nether".to_string(),
            MinecraftDimensions::End => "the_end".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameRuleValue {
    Bool(bool),
    Int(i32),
}

impl ToString for GameRuleValue {
    fn to_string(&self) -> String {
        match self {
            GameRuleValue::Bool(b) => format!("{}", b),
            GameRuleValue::Int(i) => format!("{}", i),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Self::Peaceful => format!("peaceful"),
            Self::Easy => format!("easy"),
            Self::Normal => format!("normal"),
            Self::Hard => format!("hard"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameRule {
    // Boolean rules
    AnnouncementAdvancement(Option<bool>),
    CommandBlockOutput(Option<bool>),
    DisableElytraMovementCheck(Option<bool>),
    DisableRaids(Option<bool>),
    DoDaylightCycle(Option<bool>),
    DoEntityDrops(Option<bool>),
    DoFireTick(Option<bool>),
    DoImmediateRespawn(Option<bool>),
    DoInsomnia(Option<bool>),
    DoLimitedCrafting(Option<bool>),
    DoMobLoot(Option<bool>),
    DoMobSpawning(Option<bool>),
    DoPatrolSpawning(Option<bool>),
    DoTileDrops(Option<bool>),
    DoTraderSpawning(Option<bool>),
    DoVinesSpread(Option<bool>),
    DoWardenSpawning(Option<bool>),
    DoWeatherCycle(Option<bool>),
    DrowningDamage(Option<bool>),
    FallDamage(Option<bool>),
    FireDamage(Option<bool>),
    FreezeDamage(Option<bool>),
    KeepInventory(Option<bool>),
    LogAdminCommands(Option<bool>),
    MobGriefing(Option<bool>),
    NaturalRegeneration(Option<bool>),
    ReducedDebugInfo(Option<bool>),
    SendCommandFeedback(Option<bool>),
    ShowDeathMessages(Option<bool>),
    SpectatorsGenerateChunks(Option<bool>),
    UniversalAnger(Option<bool>),

    // Integer rules
    MaxCommandChainLength(Option<i32>),
    MaxEntityCramming(Option<i32>),
    PlayersSleepingPercentage(Option<i32>),
    RandomTickSpeed(Option<i32>),
    SpawnRadius(Option<i32>),
}

impl GameRule {
    pub fn name(&self) -> &'static str {
        match self {
            GameRule::AnnouncementAdvancement(_) => "announceAdvancements",
            GameRule::CommandBlockOutput(_) => "commandBlockOutput",
            GameRule::DisableElytraMovementCheck(_) => "disableElytraMovementCheck",
            GameRule::DisableRaids(_) => "disableRaids",
            GameRule::DoDaylightCycle(_) => "doDaylightCycle",
            GameRule::DoEntityDrops(_) => "doEntityDrops",
            GameRule::DoFireTick(_) => "doFireTick",
            GameRule::DoImmediateRespawn(_) => "doImmediateRespawn",
            GameRule::DoInsomnia(_) => "doInsomnia",
            GameRule::DoLimitedCrafting(_) => "doLimitedCrafting",
            GameRule::DoMobLoot(_) => "doMobLoot",
            GameRule::DoMobSpawning(_) => "doMobSpawning",
            GameRule::DoPatrolSpawning(_) => "doPatrolSpawning",
            GameRule::DoTileDrops(_) => "doTileDrops",
            GameRule::DoTraderSpawning(_) => "doTraderSpawning",
            GameRule::DoVinesSpread(_) => "doVinesSpread",
            GameRule::DoWardenSpawning(_) => "doWardenSpawning",
            GameRule::DoWeatherCycle(_) => "doWeatherCycle",
            GameRule::DrowningDamage(_) => "drowningDamage",
            GameRule::FallDamage(_) => "fallDamage",
            GameRule::FireDamage(_) => "fireDamage",
            GameRule::FreezeDamage(_) => "freezeDamage",
            GameRule::KeepInventory(_) => "keepInventory",
            GameRule::LogAdminCommands(_) => "logAdminCommands",
            GameRule::MobGriefing(_) => "mobGriefing",
            GameRule::NaturalRegeneration(_) => "naturalRegeneration",
            GameRule::ReducedDebugInfo(_) => "reducedDebugInfo",
            GameRule::SendCommandFeedback(_) => "sendCommandFeedback",
            GameRule::ShowDeathMessages(_) => "showDeathMessages",
            GameRule::SpectatorsGenerateChunks(_) => "spectatorsGenerateChunks",
            GameRule::UniversalAnger(_) => "universalAnger",
            GameRule::MaxCommandChainLength(_) => "maxCommandChainLength",
            GameRule::MaxEntityCramming(_) => "maxEntityCramming",
            GameRule::PlayersSleepingPercentage(_) => "playersSleepingPercentage",
            GameRule::RandomTickSpeed(_) => "randomTickSpeed",
            GameRule::SpawnRadius(_) => "spawnRadius",
        }
    }

    pub fn value(&self) -> Option<GameRuleValue> {
        match self {
            // Boolean rules
            GameRule::AnnouncementAdvancement(v) => v.map(GameRuleValue::Bool),
            GameRule::CommandBlockOutput(v) => v.map(GameRuleValue::Bool),
            GameRule::DisableElytraMovementCheck(v) => v.map(GameRuleValue::Bool),
            GameRule::DisableRaids(v) => v.map(GameRuleValue::Bool),
            GameRule::DoDaylightCycle(v) => v.map(GameRuleValue::Bool),
            GameRule::DoEntityDrops(v) => v.map(GameRuleValue::Bool),
            GameRule::DoFireTick(v) => v.map(GameRuleValue::Bool),
            GameRule::DoImmediateRespawn(v) => v.map(GameRuleValue::Bool),
            GameRule::DoInsomnia(v) => v.map(GameRuleValue::Bool),
            GameRule::DoLimitedCrafting(v) => v.map(GameRuleValue::Bool),
            GameRule::DoMobLoot(v) => v.map(GameRuleValue::Bool),
            GameRule::DoMobSpawning(v) => v.map(GameRuleValue::Bool),
            GameRule::DoPatrolSpawning(v) => v.map(GameRuleValue::Bool),
            GameRule::DoTileDrops(v) => v.map(GameRuleValue::Bool),
            GameRule::DoTraderSpawning(v) => v.map(GameRuleValue::Bool),
            GameRule::DoVinesSpread(v) => v.map(GameRuleValue::Bool),
            GameRule::DoWardenSpawning(v) => v.map(GameRuleValue::Bool),
            GameRule::DoWeatherCycle(v) => v.map(GameRuleValue::Bool),
            GameRule::DrowningDamage(v) => v.map(GameRuleValue::Bool),
            GameRule::FallDamage(v) => v.map(GameRuleValue::Bool),
            GameRule::FireDamage(v) => v.map(GameRuleValue::Bool),
            GameRule::FreezeDamage(v) => v.map(GameRuleValue::Bool),
            GameRule::KeepInventory(v) => v.map(GameRuleValue::Bool),
            GameRule::LogAdminCommands(v) => v.map(GameRuleValue::Bool),
            GameRule::MobGriefing(v) => v.map(GameRuleValue::Bool),
            GameRule::NaturalRegeneration(v) => v.map(GameRuleValue::Bool),
            GameRule::ReducedDebugInfo(v) => v.map(GameRuleValue::Bool),
            GameRule::SendCommandFeedback(v) => v.map(GameRuleValue::Bool),
            GameRule::ShowDeathMessages(v) => v.map(GameRuleValue::Bool),
            GameRule::SpectatorsGenerateChunks(v) => v.map(GameRuleValue::Bool),
            GameRule::UniversalAnger(v) => v.map(GameRuleValue::Bool),

            // Integer rules
            GameRule::MaxCommandChainLength(v) => v.map(GameRuleValue::Int),
            GameRule::MaxEntityCramming(v) => v.map(GameRuleValue::Int),
            GameRule::PlayersSleepingPercentage(v) => v.map(GameRuleValue::Int),
            GameRule::RandomTickSpeed(v) => v.map(GameRuleValue::Int),
            GameRule::SpawnRadius(v) => v.map(GameRuleValue::Int),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeatherType {
    Clear(TickDuration),
    Thunder(TickDuration),
    Rain(TickDuration),
}

impl ToString for WeatherType {
    fn to_string(&self) -> String {
        match self {
            WeatherType::Clear(duration) => format!("clear {}", duration),
            WeatherType::Thunder(duration) => format!("thunder {}", duration),
            WeatherType::Rain(duration) => format!("rain {}", duration),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gamemode {
    Creative,
    Survival,
    Adventure,
    Spectator,
}

impl ToString for Gamemode {
    fn to_string(&self) -> String {
        match self {
            Gamemode::Creative => "creative".to_string(),
            Gamemode::Survival => "survival".to_string(),
            Gamemode::Adventure => "adventure".to_string(),
            Gamemode::Spectator => "spectator".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Distance {
    Exact(f32),      // A single exact distance
    Range(f32, f32), // A range with minimum and maximum distance
    Max(f32),        // Maximum distance (e.g., `..10`)
    Min(f32),        // Minimum distance (e.g., `3..`)
}

impl ToString for Distance {
    fn to_string(&self) -> String {
        match self {
            Distance::Exact(distance) => format!("{}", distance),
            Distance::Range(min, max) => format!("{}..{}", min, max),
            Distance::Max(max) => format!("..{}", max),
            Distance::Min(min) => format!("{}..", min),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Location {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeleportTargetLocation {
    pub pos: Option<Location>,
    pub entity: Option<super::selector::TargetSelector>,
}

impl ToString for TeleportTargetLocation {
    fn to_string(&self) -> String {
        if let Some(pos) = &self.pos {
            format!("{}", pos.to_string())
        } else if let Some(entity) = &self.entity {
            format!("{}", entity.to_string())
        } else {
            panic!("Invalid teleport target location")
        }
    }
}

#[derive(Debug, Clone)]
pub enum TimeAction {
    Add(u32),
    Set(TimeValue),
    Query(TimeQueryType),
}

impl ToString for TimeAction {
    fn to_string(&self) -> String {
        match self {
            TimeAction::Add(ticks) => format!("time add {}", ticks),
            TimeAction::Set(value) => format!(
                "time set {}",
                match value {
                    TimeValue::Day => "day".to_string(),
                    TimeValue::Night => "night".to_string(),
                    TimeValue::Noon => "noon".to_string(),
                    TimeValue::Midnight => "midnight".to_string(),
                    TimeValue::Ticks(t) => t.to_string(),
                }
            ),
            TimeAction::Query(query_type) => format!(
                "time query {}",
                match query_type {
                    TimeQueryType::DayTime => "daytime",
                    TimeQueryType::GameTime => "gametime",
                    TimeQueryType::Day => "day",
                }
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TimeValue {
    Day,
    Night,
    Noon,
    Midnight,
    Ticks(u32),
}

impl ToString for TimeValue {
    fn to_string(&self) -> String {
        match self {
            TimeValue::Day => "day".to_string(),
            TimeValue::Night => "night".to_string(),
            TimeValue::Noon => "noon".to_string(),
            TimeValue::Midnight => "midnight".to_string(),
            TimeValue::Ticks(ticks) => format!("{}", ticks),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TimeQueryType {
    DayTime,
    GameTime,
    Day,
}

impl ToString for TimeQueryType {
    fn to_string(&self) -> String {
        match self {
            TimeQueryType::DayTime => "daytime".to_string(),
            TimeQueryType::GameTime => "gametime".to_string(),
            TimeQueryType::Day => "day".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorldBorderAction {
    Add {
        distance: f32,
        time: Option<u32>,
    },
    Set {
        distance: f32,
        time: Option<u32>,
    },
    Center {
        x: f32,
        z: f32,
    },
    DamageAmount {
        damage: f32,
    },
    DamageBuffer {
        distance: f32,
    },
    Get,
    Warning {
        warning_type: WorldBorderWarning,
        distance: u32,
    },
}

#[derive(Debug, Clone)]
pub enum WorldBorderWarning {
    Distance,
    Time,
}

impl ToString for WorldBorderWarning {
    fn to_string(&self) -> String {
        match self {
            WorldBorderWarning::Distance => "distance".to_string(),
            WorldBorderWarning::Time => "time".to_string(),
        }
    }
}
