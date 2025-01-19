use crate::types::permissions::WhitelistAction;

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Whitelist(WhitelistAction);

impl Whitelist {
    pub fn new(action: WhitelistAction) -> Self {
        Self(action)
    }
}

impl ToString for Whitelist {
    fn to_string(&self) -> String {
        format!("whitelist {}", self.0.to_string())
    }
}

impl MinecraftCommand for Whitelist {}
