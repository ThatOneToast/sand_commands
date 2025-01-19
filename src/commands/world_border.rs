use crate::types::world::WorldBorderAction;

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct WorldBorder {
    pub action: crate::types::world::WorldBorderAction,
}

impl WorldBorder {
    pub fn new(action: crate::types::world::WorldBorderAction) -> Self {
        Self { action }
    }
}

impl ToString for WorldBorder {
    fn to_string(&self) -> String {
        match &self.action {
            WorldBorderAction::Add { distance, time } => {
                if let Some(t) = time {
                    format!("worldborder add {} {}", distance, t)
                } else {
                    format!("worldborder add {}", distance)
                }
            }
            WorldBorderAction::Set { distance, time } => {
                if let Some(t) = time {
                    format!("worldborder set {} {}", distance, t)
                } else {
                    format!("worldborder set {}", distance)
                }
            }
            WorldBorderAction::Center { x, z } => {
                format!("worldborder center {} {}", x, z)
            }
            WorldBorderAction::DamageAmount { damage } => {
                format!("worldborder damage amount {}", damage)
            }
            WorldBorderAction::DamageBuffer { distance } => {
                format!("worldborder damage buffer {}", distance)
            }
            WorldBorderAction::Get => "worldborder get".to_string(),
            WorldBorderAction::Warning { warning_type, distance } => {
                format!("worldborder warning {} {}", warning_type.to_string(), distance)
            }
        }
    }
}

impl MinecraftCommand for WorldBorder {}
