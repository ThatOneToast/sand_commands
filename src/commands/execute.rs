use std::sync::Arc;

use crate::{
    prelude::MinecraftEntity,
    types::{HeightMap, Location, MinecraftComparative, MinecraftDimension, MinecraftDimensions, MinecraftType, TargetSelector, Yaw},
};

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub enum ExecuteRelation {
    ATTACKER,
    CONTROLLER,
    LEASHER,
    ORIGIN,
    OWNER,
    PASSENGERS,
    TARGET,
    VEHICLE,
}

impl ToString for ExecuteRelation {
    fn to_string(&self) -> String {
        match self {
            ExecuteRelation::ATTACKER => "attacker".to_string(),
            ExecuteRelation::CONTROLLER => "controller".to_string(),
            ExecuteRelation::LEASHER => "leasher".to_string(),
            ExecuteRelation::ORIGIN => "origin".to_string(),
            ExecuteRelation::OWNER => "owner".to_string(),
            ExecuteRelation::PASSENGERS => "passengers".to_string(),
            ExecuteRelation::TARGET => "target".to_string(),
            ExecuteRelation::VEHICLE => "vehicle".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteAnchor {
    Eyes,
    Feet,
}

impl ToString for ExecuteAnchor {
    fn to_string(&self) -> String {
        match self {
            ExecuteAnchor::Eyes => "eyes".to_string(),
            ExecuteAnchor::Feet => "feet".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecutePositioned {
    POS { x: f32, y: f32, z: f32 },
    TARGETS(TargetSelector),
    HEIGHTMAP(HeightMap),
}

impl ToString for ExecutePositioned {
    fn to_string(&self) -> String {
        match self {
            ExecutePositioned::POS { x, y, z } => format!("{} {} {}", x, y, z),
            ExecutePositioned::TARGETS(ts) => format!("as {}", ts.to_string()),
            ExecutePositioned::HEIGHTMAP(hm) => format!("over {}", hm.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteFacing {
    POS { x: f32, y: f32, z: f32 },
    TARGETS(TargetSelector),
}

impl ToString for ExecuteFacing {
    fn to_string(&self) -> String {
        match self {
            ExecuteFacing::POS { x, y, z } => format!("{} {} {}", x, y, z),
            ExecuteFacing::TARGETS(ts) => format!("entity {}", ts.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteRotation {
    TARGETS(TargetSelector),
    ROT(Yaw, f32),
}

impl ToString for ExecuteRotation {
    fn to_string(&self) -> String {
        match self {
            ExecuteRotation::TARGETS(ts) => format!("as {}", ts.to_string()),
            ExecuteRotation::ROT(yaw, pitch) => format!("{} {}", yaw.to_string(), pitch),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteStoreAction {
    RESULT,
    SUCCESS,
}

impl ToString for ExecuteStoreAction {
    fn to_string(&self) -> String {
        match self {
            ExecuteStoreAction::RESULT => "result".to_string(),
            ExecuteStoreAction::SUCCESS => "success".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteBossBarValue {
    VALUE,
    MAX,
}

impl ToString for ExecuteBossBarValue {
    fn to_string(&self) -> String {
        match self {
            ExecuteBossBarValue::VALUE => "value".to_string(),
            ExecuteBossBarValue::MAX => "max".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteStoreCommand {
    BLOCK {
        target_pos: Location,
        nbt_path: String,
        data_type: MinecraftType,
        scale: f64,
    },
    BOSSBAR {
        resource_location: String,
        value: ExecuteBossBarValue,
    },
    ENTITY {
        target: TargetSelector,
        nbt_path: String,
        data_type: MinecraftType,
        scale: f64,
    },
    SCORE {
        target: TargetSelector,
        objective: String,
    },
    STORAGE {
        resource_location: String,
        nbt_path: String,
        data_type: MinecraftType,
        scale: f64,
    },
}

impl ToString for ExecuteStoreCommand {
    fn to_string(&self) -> String {
        match self {
            ExecuteStoreCommand::BLOCK {
                target_pos,
                nbt_path,
                data_type,
                scale,
            } => {
                format!(
                    "block {} {} {} {}",
                    target_pos.to_string(),
                    nbt_path,
                    data_type.to_string(),
                    scale
                )
            }
            ExecuteStoreCommand::BOSSBAR {
                resource_location,
                value,
            } => {
                format!("bossbar {} {}", resource_location, value.to_string())
            }
            ExecuteStoreCommand::ENTITY {
                target,
                nbt_path,
                data_type,
                scale,
            } => {
                format!(
                    "entity {} {} {} {}",
                    target.to_string(),
                    nbt_path,
                    data_type.to_string(),
                    scale
                )
            }
            ExecuteStoreCommand::SCORE { target, objective } => {
                format!("score {} {}", target.to_string(), objective)
            }
            ExecuteStoreCommand::STORAGE {
                resource_location,
                nbt_path,
                data_type,
                scale,
            } => {
                format!(
                    "storage {} {} {} {}",
                    resource_location,
                    nbt_path,
                    data_type.to_string(),
                    scale
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteScanMode {
    ALL,
    MASKED
}

impl ToString for ExecuteScanMode {
    fn to_string(&self) -> String {
        match self {
            ExecuteScanMode::ALL => "all".to_string(),
            ExecuteScanMode::MASKED => "masked".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteDataCommand {
    BLOCK {
        pos: Location,
        nbt_path: String,
    },
    ENTITY {
        target: TargetSelector,
        nbt_path: String,
    },
    STORAGE {
        resource_location: String,
        nbt_path: String,
    }
}

impl ToString for ExecuteDataCommand {
    fn to_string(&self) -> String {
        match self {
            ExecuteDataCommand::BLOCK { pos, nbt_path } => {
                format!("block {} {}", pos.to_string(), nbt_path)
            },
            ExecuteDataCommand::ENTITY { target, nbt_path } => {
                format!("entity {} {}", target.to_string(), nbt_path)
            },
            ExecuteDataCommand::STORAGE { resource_location, nbt_path } => {
                format!("storage {} {}", resource_location, nbt_path)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteItemsCommand {
    BLOCK {
        source_pos: Location,
        slots: String,
        predicate: String
    },
    ENTITY {
        source: TargetSelector,
        slots: String,
        predicate: String
    }
}

impl ToString for ExecuteItemsCommand {
    fn to_string(&self) -> String {
        match self {
            ExecuteItemsCommand::BLOCK { source_pos, slots, predicate } => {
                format!("block {} {} {}", source_pos.to_string(), slots, predicate)
            },
            ExecuteItemsCommand::ENTITY { source, slots, predicate } => {
                format!("entity {} {} {}", source.to_string(), slots, predicate)
            }
        }
    } 
}

#[derive(Debug, Clone)]
pub enum ExecuteScoreCommand {
    MATCHES {
        target: TargetSelector,
        objective: String,
        range: String,
    },
    COMPARE {
        target: TargetSelector,
        objective: String,
        comparative: MinecraftComparative,
        source: TargetSelector,
        source_objective: String,
    }
}

impl ToString for ExecuteScoreCommand {
    fn to_string(&self) -> String {
        match self {
            ExecuteScoreCommand::MATCHES { target, objective, range } => {
                format!("matches {} \"{}\" {}", objective, target.to_string(), range)
            },
            ExecuteScoreCommand::COMPARE { target, objective, comparative, source, source_objective } => {
                format!("{} {} {} {} {}", target.to_string(), objective, comparative.to_string(), source.to_string(), source_objective)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExecuteConditionCategory {
    BIOME {
        pos: Location,
        biome: String,
    },
    BLOCK {
        pos: Location,
        predicate: String
    },
    BLOCKS {
        start: Location,
        end: Location,
        destination: Location,
        scan_mode: ExecuteScanMode
    },
    DATA(ExecuteDataCommand),
    DIMENSION(MinecraftDimensions),
    ENTITY(TargetSelector),
    FUNCTION(String),
    ITEMS(ExecuteItemsCommand),
    LOADED(Location),
    PREDICATE(String),
    SCORE(ExecuteScoreCommand)
} 

impl ToString for ExecuteConditionCategory {
    fn to_string(&self) -> String {
        match self {
            ExecuteConditionCategory::BIOME { pos, biome } => {
                format!("biome {} {}", pos.to_string(), biome)
            },
            ExecuteConditionCategory::BLOCK { pos, predicate } => {
                format!("block {} {}", pos.to_string(), predicate)
            },
            ExecuteConditionCategory::BLOCKS { start, end, destination, scan_mode } => {
                format!("blocks {} {} {} {}", start.to_string(), end.to_string(), destination.to_string(), scan_mode.to_string())
            },
            ExecuteConditionCategory::DATA(data) => {
                format!("data {}", data.to_string())
            },
            ExecuteConditionCategory::DIMENSION(dim) => {
                format!("dimension {}", dim.to_string())
            },
            ExecuteConditionCategory::ENTITY(entity) => {
                format!("entity {}", entity.to_string())
            },
            ExecuteConditionCategory::FUNCTION(function) => {
                format!("function {}", function)
            },
            ExecuteConditionCategory::ITEMS(items) => {
                format!("items {}", items.to_string())
            },
            ExecuteConditionCategory::LOADED(location) => {
                format!("loaded {}", location.to_string())
            },
            ExecuteConditionCategory::PREDICATE(predicate) => {
                format!("predicate {}", predicate)
            },
            ExecuteConditionCategory::SCORE(score) => {
                format!("score {}", score.to_string())
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum ExecuteSubCommand {
    ALIGN(String, Box<ExecuteSubCommand>),
    ANCHORED(ExecuteAnchor, Box<ExecuteSubCommand>),
    AS(TargetSelector, Box<ExecuteSubCommand>),
    AT(TargetSelector, Box<ExecuteSubCommand>),
    FACING(ExecuteFacing, Box<ExecuteSubCommand>),
    IN(Arc<dyn MinecraftDimension>, Box<ExecuteSubCommand>),
    ON(ExecuteRelation, Box<ExecuteSubCommand>),
    POSITIONED(ExecutePositioned, Box<ExecuteSubCommand>),
    ROTATED(ExecuteRotation, Box<ExecuteSubCommand>),
    STORE(
        ExecuteStoreAction,
        ExecuteStoreCommand,
        Box<ExecuteSubCommand>,
    ),
    SUMMON(MinecraftEntity),
    IF(ExecuteConditionCategory, Box<ExecuteSubCommand>),
    UNLESS(ExecuteConditionCategory, Box<ExecuteSubCommand>),
    RUN(Arc<dyn MinecraftCommand>),
}

impl ToString for ExecuteSubCommand {
    fn to_string(&self) -> String {
        match self {
            ExecuteSubCommand::ALIGN(axis, command) => {
                format!("align {} {}", axis, command.to_string())
            }
            ExecuteSubCommand::ANCHORED(anch, command) => {
                format!("anchored {} {}", anch.to_string(), command.to_string())
            }
            ExecuteSubCommand::AS(ts, command) => {
                format!("as {} {}", ts.to_string(), command.to_string())
            }
            ExecuteSubCommand::AT(ts, command) => {
                format!("at {} {}", ts.to_string(), command.to_string())
            }
            ExecuteSubCommand::FACING(fac, command) => {
                format!("facing {} {}", fac.to_string(), command.to_string())
            }
            ExecuteSubCommand::IN(dem, command) => {
                format!("in {} {}", dem.to_string(), command.to_string())
            }
            ExecuteSubCommand::ON(rel, command) => {
                format!("on {} {}", rel.to_string(), command.to_string())
            }
            ExecuteSubCommand::POSITIONED(pos, command) => {
                format!("positioned {} {}", pos.to_string(), command.to_string())
            }
            ExecuteSubCommand::ROTATED(rot, command) => {
                format!("rotated {} {}", rot.to_string(), command.to_string())
            }
            ExecuteSubCommand::STORE(action, store, command) => format!(
                "store {} {} {}",
                action.to_string(),
                store.to_string(),
                command.to_string()
            ),
            ExecuteSubCommand::SUMMON(entity) => format!("summon {}", entity.to_string()),
            ExecuteSubCommand::IF(condition, command) => format!("if {} {}", condition.to_string(), command.to_string()),
            ExecuteSubCommand::UNLESS(condition, command) => format!("unless {} {}", condition.to_string(), command.to_string()),
            ExecuteSubCommand::RUN(command) => format!("run {}", command.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Execute(pub ExecuteSubCommand);
impl MinecraftCommand for Execute {}

impl ToString for Execute {
    fn to_string(&self) -> String {
        format!("execute {}", self.0.to_string())
    }
}
