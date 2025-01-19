use super::{ban::BanTarget, MinecraftCommand};

#[derive(Debug, Clone)]
pub struct UnBan(BanTarget);

impl UnBan {
    pub fn new(bt: BanTarget) -> Self {
        UnBan(bt)
    }
}

impl ToString for UnBan {
    fn to_string(&self) -> String {
        match &self.0 {
            BanTarget::IP(addr) => format!("pardon-ip {}", addr),
            BanTarget::Player(name) => format!("pardon {}", name),
        }
    }
}


impl MinecraftCommand for UnBan {}
