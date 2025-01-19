use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Teleport {
    pub teleporting_self: bool,
    pub target: Option<crate::types::selector::TargetSelector>,
    pub location: crate::types::world::TeleportTargetLocation,
}

impl ToString for Teleport {
    fn to_string(&self) -> String {
        if self.teleporting_self {
            format!("tp @s {}", self.location.to_string())
        } else if let Some(target) = &self.target {
            format!("tp {} {}", target.to_string(), self.location.to_string())
        } else {
            panic!("Invalid teleport target")
        }
    }
}


impl MinecraftCommand for Teleport {}
