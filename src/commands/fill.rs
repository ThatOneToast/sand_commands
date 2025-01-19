use crate::types::Location;

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Fill {
    pub from: Location,
    pub to: Location,
    pub with: String,
}

impl ToString for Fill {
    fn to_string(&self) -> String {
        format!(
            "fill {} {} {}",
            self.from.to_string(),
            self.to.to_string(),
            self.with
        )
    }
}

impl MinecraftCommand for Fill {}
