#[derive(Debug, Clone)]
pub enum ScoreboardAction {
    Objectives(ObjectiveAction),
    Players(PlayerAction),
}

#[derive(Debug, Clone)]
pub enum ObjectiveAction {
    Add {
        objective: String,
        criteria: ObjectiveCriteria,
        display_name: Option<String>,
    },
    Remove {
        objective: String,
    },
    SetDisplay {
        display_slot: DisplaySlot,
        objective: Option<String>,
    },
    Modify {
        objective: String,
        modifier: ObjectiveModifier,
    },
    List,
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    Add {
        target: crate::types::selector::TargetSelector,
        objective: String,
        score: i32,
    },
    Remove {
        target: crate::types::selector::TargetSelector,
        objective: String,
    },
    Set {
        target: crate::types::selector::TargetSelector,
        objective: String,
        score: i32,
    },
    Reset {
        target: crate::types::selector::TargetSelector,
        objective: Option<String>,
    },
    Enable {
        target: crate::types::selector::TargetSelector,
        objective: String,
    },
    Operation {
        target: crate::types::selector::TargetSelector,
        target_objective: String,
        operation: Operation,
        source: crate::types::selector::TargetSelector,
        source_objective: String,
    },
    Get {
        target: crate::types::selector::TargetSelector,
        objective: String,
    },
    List {
        target: Option<crate::types::selector::TargetSelector>,
    },
}

#[derive(Debug, Clone)]
pub enum TriggerAction {
    Add(i32),
    Set(i32),
    Simply, // Just "trigger <objective>"
}

#[derive(Debug, Clone)]
pub enum Operation {
    Assign,       // =
    PlusEquals,   // +=
    MinusEquals,  // -=
    TimesEquals,  // *=
    DivideEquals, // /=
    ModEquals,    // %=
    Min,          // <
    Max,          // >
    Swap,         // ><
}

#[derive(Debug, Clone)]
pub enum ObjectiveCriteria {
    Dummy,
    Trigger,
    DeathCount,
    PlayerKillCount,
    TotalKillCount,
    Health,
    XP,
    Level,
    Food,
    Air,
    Armor,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum DisplaySlot {
    List,
    Sidebar,
    BelowName,
    SidebarTeam(String), // For team-specific sidebars
}

#[derive(Debug, Clone)]
pub enum ObjectiveModifier {
    DisplayName(String),
    RenderType(RenderType),
}

#[derive(Debug, Clone)]
pub enum RenderType {
    Hearts,
    Integer,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Assign => "=".to_string(),
            Operation::PlusEquals => "+=".to_string(),
            Operation::MinusEquals => "-=".to_string(),
            Operation::TimesEquals => "*=".to_string(),
            Operation::DivideEquals => "/=".to_string(),
            Operation::ModEquals => "%=".to_string(),
            Operation::Min => "<".to_string(),
            Operation::Max => ">".to_string(),
            Operation::Swap => "><".to_string(),
        }
    }
}

impl ToString for ObjectiveCriteria {
    fn to_string(&self) -> String {
        match self {
            ObjectiveCriteria::Dummy => "dummy".to_string(),
            ObjectiveCriteria::Trigger => "trigger".to_string(),
            ObjectiveCriteria::DeathCount => "deathCount".to_string(),
            ObjectiveCriteria::PlayerKillCount => "playerKillCount".to_string(),
            ObjectiveCriteria::TotalKillCount => "totalKillCount".to_string(),
            ObjectiveCriteria::Health => "health".to_string(),
            ObjectiveCriteria::XP => "xp".to_string(),
            ObjectiveCriteria::Level => "level".to_string(),
            ObjectiveCriteria::Food => "food".to_string(),
            ObjectiveCriteria::Air => "air".to_string(),
            ObjectiveCriteria::Armor => "armor".to_string(),
            ObjectiveCriteria::Custom(s) => s.clone(),
        }
    }
}

impl ToString for DisplaySlot {
    fn to_string(&self) -> String {
        match self {
            DisplaySlot::List => "list".to_string(),
            DisplaySlot::Sidebar => "sidebar".to_string(),
            DisplaySlot::BelowName => "belowName".to_string(),
            DisplaySlot::SidebarTeam(team) => format!("sidebar.team.{}", team),
        }
    }
}

impl ToString for RenderType {
    fn to_string(&self) -> String {
        match self {
            RenderType::Hearts => "hearts".to_string(),
            RenderType::Integer => "integer".to_string(),
        }
    }
}
