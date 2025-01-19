use crate::types::PlayerName;

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Ban(BanTarget);

impl Ban {
    pub fn new(target: BanTarget) -> Self {
        Ban(target)
    }
}

impl ToString for Ban {
    fn to_string(&self) -> String {
        match &self.0 {
            BanTarget::Player(target) => format!("ban {}", target.to_owned()),
            BanTarget::IP(address) => format!("ban-ip {}", address.to_owned())
        }
    }
}

impl MinecraftCommand for Ban {}

#[derive(Debug, Clone)]
pub enum BanTarget {
    Player(PlayerName),
    IP(String)
}

