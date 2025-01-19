use super::world::PlayerName;

#[derive(Debug, Clone)]
pub enum WhitelistAction {
    Add(PlayerName),
    Remove(PlayerName),
    List,
    Off,
    On,
    Reload,
}

impl ToString for WhitelistAction {
    fn to_string(&self) -> String {
        match self {
            WhitelistAction::Add(name) => format!("add {}", name),
            WhitelistAction::Remove(name) => format!("remove {}", name),
            WhitelistAction::List => "list".to_string(),
            WhitelistAction::Off => "off".to_string(),
            WhitelistAction::On => "on".to_string(),
            WhitelistAction::Reload => "reload".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OPAction {
    GRANT(PlayerName),
    REVOKE(PlayerName)
}

impl ToString for OPAction {
    fn to_string(&self) -> String {
        match &self {
           &Self::GRANT(name) => format!("op {}", name),
           &Self::REVOKE(name) => format!("deop {}", name)
        }
    }
}