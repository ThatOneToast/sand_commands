use crate::types::Location;

use super::MinecraftCommand;


#[derive(Debug, Clone)]
pub struct Setblock {
    pub at: Location,
    pub with: String
}

impl ToString for Setblock {
    fn to_string(&self) -> String {
        format!("setblock {} {}", self.at.to_string(), self.with)
    }
}

impl MinecraftCommand for Setblock {}
