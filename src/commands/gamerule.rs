use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct GameRule {
    pub rule: crate::types::world::GameRule,
}

impl GameRule {
    pub fn new(rule: crate::types::world::GameRule) -> Self {
        Self { rule }
    }
}

impl ToString for GameRule {
    fn to_string(&self) -> String {
        match self.rule.value() {
            Some(value) => format!("gamerule {} {}", self.rule.name(), value.to_string()),
            None => format!("gamerule {}", self.rule.name()),
        }
    }
}

impl MinecraftCommand for GameRule {}
