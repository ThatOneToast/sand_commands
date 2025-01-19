use crate::types::scoreboard::{ObjectiveAction, ObjectiveModifier, PlayerAction, ScoreboardAction};

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Scoreboard {
    pub action: ScoreboardAction,
}

impl Scoreboard {
    pub fn new(action: ScoreboardAction) -> Self {
        Self { action }
    }
}

impl ToString for Scoreboard {
    fn to_string(&self) -> String {
        match &self.action {
            ScoreboardAction::Objectives(obj_action) => match obj_action {
                ObjectiveAction::Add {
                    objective,
                    criteria,
                    display_name,
                } => {
                    if let Some(name) = display_name {
                        format!(
                            "scoreboard objectives add {} {} \"{}\"",
                            objective,
                            criteria.to_string(),
                            name
                        )
                    } else {
                        format!(
                            "scoreboard objectives add {} {}",
                            objective,
                            criteria.to_string()
                        )
                    }
                }
                ObjectiveAction::Remove { objective } => {
                    format!("scoreboard objectives remove {}", objective)
                }
                ObjectiveAction::SetDisplay {
                    display_slot,
                    objective,
                } => {
                    if let Some(obj) = objective {
                        format!(
                            "scoreboard objectives setdisplay {} {}",
                            display_slot.to_string(),
                            obj
                        )
                    } else {
                        format!(
                            "scoreboard objectives setdisplay {}",
                            display_slot.to_string()
                        )
                    }
                }
                ObjectiveAction::Modify {
                    objective,
                    modifier,
                } => match modifier {
                    ObjectiveModifier::DisplayName(name) => {
                        format!(
                            "scoreboard objectives modify {} displayname {}",
                            objective, name
                        )
                    }
                    ObjectiveModifier::RenderType(render_type) => {
                        format!(
                            "scoreboard objectives modify {} rendertype {}",
                            objective,
                            render_type.to_string()
                        )
                    }
                },
                ObjectiveAction::List => "scoreboard objectives list".to_string(),
            },
            ScoreboardAction::Players(player_action) => match player_action {
                PlayerAction::Add {
                    target,
                    objective,
                    score,
                } => {
                    format!(
                        "scoreboard players add {} {} {}",
                        target.to_string(),
                        objective,
                        score
                    )
                }
                PlayerAction::Remove { target, objective } => {
                    format!(
                        "scoreboard players remove {} {}",
                        target.to_string(),
                        objective
                    )
                }
                PlayerAction::Set {
                    target,
                    objective,
                    score,
                } => {
                    format!(
                        "scoreboard players set {} {} {}",
                        target.to_string(),
                        objective,
                        score
                    )
                }
                PlayerAction::Reset { target, objective } => {
                    if let Some(obj) = objective {
                        format!("scoreboard players reset {} {}", target.to_string(), obj)
                    } else {
                        format!("scoreboard players reset {}", target.to_string())
                    }
                }
                PlayerAction::Enable { target, objective } => {
                    format!(
                        "scoreboard players enable {} {}",
                        target.to_string(),
                        objective
                    )
                }
                PlayerAction::Operation {
                    target,
                    target_objective,
                    operation,
                    source,
                    source_objective,
                } => {
                    format!(
                        "scoreboard players operation {} {} {} {} {}",
                        target.to_string(),
                        target_objective,
                        operation.to_string(),
                        source.to_string(),
                        source_objective
                    )
                }
                PlayerAction::Get { target, objective } => {
                    format!(
                        "scoreboard players get {} {}",
                        target.to_string(),
                        objective
                    )
                }
                PlayerAction::List { target } => {
                    if let Some(t) = target {
                        format!("scoreboard players list {}", t.to_string())
                    } else {
                        "scoreboard players list".to_string()
                    }
                }
            },
        }
    }
}

impl MinecraftCommand for Scoreboard {}
