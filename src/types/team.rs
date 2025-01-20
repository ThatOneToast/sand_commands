use crate::types::TextComponent;

pub type TeamName = String;

#[derive(Debug, Clone)]
pub enum TeamAction {
    /// Returns a list of all teams
    LIST,
    /// Creates a team with the given name and optional display name. <displayName> defaults to TeamName when unspecified.
    ADD(TeamName, Option<TextComponent>),
    ///  Deletes the specified team.
    REMOVE(TeamName),
    /// Removes all members from the named team.
    EMPTY(TeamName),
    /// Assigns the specified entities to the specified team. If no entities are specified, makes the command executor join the team.
    JOIN(TeamName, Option<Vec<super::selector::TargetSelector>>),
    /// Makes the specified entities leave their teams.
    LEAVE(Option<Vec<super::selector::TargetSelector>>),
    /// Modifies the options of the specified team.
    MODIFY(TeamName, TeamActionModify),
}

impl ToString for TeamAction {
    fn to_string(&self) -> String {
        match self {
            TeamAction::LIST => "list".to_string(),
            TeamAction::ADD(name, display_name) => {
                format!(
                    "add {} {}",
                    name,
                    match display_name.as_ref() {
                        Some(display_name) => display_name.to_string(),
                        None => TextComponent::String(name.to_owned()).to_string(),
                    },
                )
            }
            TeamAction::REMOVE(name) => format!("remove {}", name),
            TeamAction::EMPTY(name) => format!("empty {}", name),
            TeamAction::JOIN(name, members) => format!(
                "join {} {}",
                name,
                match members {
                    Some(members) => members
                        .into_iter()
                        .map(|m| m.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    None => "@s".to_string(),
                },
            ),
            TeamAction::LEAVE(members) => format!(
                "leave {}",
                match members {
                    Some(members) => members
                        .into_iter()
                        .map(|m| m.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    None => "@s".to_string(),
                }
            ),
            TeamAction::MODIFY(name, modify) => format!("modify {} {}", name, modify.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TeamActionModify {
    /// Sets the team's display name.
    DisplayName(TextComponent),
    /// Sets the team's prefix.
    Prefix(String),
    /// Sets the team's suffix.
    Suffix(String),
    /// Sets the team's color.
    Color(String),
    /// Sets the team's nametag visibility.
    NametagVisibility(TeamNametagVisibility),
    /// Sets the team's collision rule.
    CollisionRule(TeamCollisionRule),
    /// Sets the team's seeFriendlyInvisibles rule.
    SeeFriendlyInvisibles(bool),
}

impl ToString for TeamActionModify {
    fn to_string(&self) -> String {
        match self {
            TeamActionModify::DisplayName(display_name) => format!("displayName {}", display_name),
            TeamActionModify::Prefix(prefix) => format!("prefix {}", prefix),
            TeamActionModify::Suffix(suffix) => format!("suffix {}", suffix),
            TeamActionModify::Color(color) => format!("color {}", color),
            TeamActionModify::NametagVisibility(visibility) => {
                format!("nametagVisibility {}", visibility.to_string())
            }
            TeamActionModify::CollisionRule(rule) => format!("collisionRule {}", rule.to_string()),
            TeamActionModify::SeeFriendlyInvisibles(see) => {
                format!("seeFriendlyInvisibles {}", see)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TeamNametagVisibility {
    Always,
    Never,
    HideForOtherTeams,
    HideForOwnTeam,
}

impl ToString for TeamNametagVisibility {
    fn to_string(&self) -> String {
        match self {
            TeamNametagVisibility::Always => "always".to_string(),
            TeamNametagVisibility::Never => "never".to_string(),
            TeamNametagVisibility::HideForOtherTeams => "hideForOtherTeams".to_string(),
            TeamNametagVisibility::HideForOwnTeam => "hideForOwnTeam".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TeamCollisionRule {
    PushOtherTeams,
    PushOwnTeam,
    Never,
    Always,
}

impl ToString for TeamCollisionRule {
    fn to_string(&self) -> String {
        match self {
            TeamCollisionRule::PushOtherTeams => "pushOtherTeams".to_string(),
            TeamCollisionRule::PushOwnTeam => "pushOwnTeam".to_string(),
            TeamCollisionRule::Never => "never".to_string(),
            TeamCollisionRule::Always => "always".to_string(),
        }
    }
}
