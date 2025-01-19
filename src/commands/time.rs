use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Time {
    pub action: crate::types::world::TimeAction,
}

impl Time {
    pub fn new(action: crate::types::world::TimeAction) -> Self {
        Self { action }
    }
}

impl ToString for Time {
    fn to_string(&self) -> String {
        match &self.action {
            crate::types::world::TimeAction::Add(ticks) => format!("time add {}", ticks),
            crate::types::world::TimeAction::Set(value) => {
                format!("time set {}", value.to_string())
            }
            crate::types::world::TimeAction::Query(query_type) => {
                format!("time query {}", query_type.to_string())
            }
        }
    }
}


impl MinecraftCommand for Time {}
