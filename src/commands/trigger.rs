use crate::types::scoreboard::TriggerAction;

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Trigger {
    pub objective: String,
    pub action: Option<crate::types::scoreboard::TriggerAction>,
}

impl Trigger {
    pub fn new(objective: String, action: Option<crate::types::scoreboard::TriggerAction>) -> Self {
        Self { objective, action }
    }
}

impl ToString for Trigger {
    fn to_string(&self) -> String {
        match &self.action {
            Some(action) => match action {
                TriggerAction::Add(value) => format!("trigger {} add {}", self.objective, value),
                TriggerAction::Set(value) => format!("trigger {} set {}", self.objective, value),
                TriggerAction::Simply => format!("trigger {}", self.objective),
            },
            None => format!("trigger {}", self.objective),
        }
    }
}


impl MinecraftCommand for Trigger {}
