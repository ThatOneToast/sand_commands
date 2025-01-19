use crate::types::data::TagAction;

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Tag {
    pub target: crate::types::selector::TargetSelector,
    pub action: TagAction,
}

impl Tag {
    pub fn new(target: crate::types::selector::TargetSelector, action: TagAction) -> Self {
        Self { target, action }
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        match &self.action {
            TagAction::Add(name) => format!("tag {} add {}", self.target.to_string(), name),
            TagAction::Remove(name) => format!("tag {} remove {}", self.target.to_string(), name),
            TagAction::List => format!("tag {} list", self.target.to_string()),
        }
    }
}


impl MinecraftCommand for Tag {}
