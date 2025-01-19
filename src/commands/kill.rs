use crate::types::TargetSelector;

use super::MinecraftCommand;


#[derive(Debug, Clone)]
pub struct Kill(Option<TargetSelector>);

impl Kill {
    pub fn new(ts: TargetSelector) -> Self {
        Kill(Some(ts))
    }
    pub fn new_self() -> Self {
        Kill(None)
    }
}

impl ToString for Kill {
    fn to_string(&self) -> String {
        if self.0.is_some() {
            return format!("kill {}", self.0.as_ref().unwrap().to_string())
        } else {
            return format!("kill")
        }
    }
}

impl MinecraftCommand for Kill {}

