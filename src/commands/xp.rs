use super::MinecraftCommand;


#[derive(Debug, Clone)]
pub struct XP {
    pub action: crate::types::experience::EXPAction,
}

impl XP {
    pub fn new(action: crate::types::experience::EXPAction) -> Self {
        Self { action }
    }
}

impl ToString for XP {
    fn to_string(&self) -> String {
        format!("xp {}", self.action.to_string())
    }
}


impl MinecraftCommand for XP {}
