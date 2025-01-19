use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct DefaultGamemode(crate::types::world::Gamemode);

impl DefaultGamemode {
    pub fn new(gm: crate::types::world::Gamemode) -> Self {
        DefaultGamemode(gm)
    }
}

impl ToString for DefaultGamemode {
    fn to_string(&self) -> String {
        format!("defaultgamemode {}", self.0.to_string())
    }
}

impl MinecraftCommand for DefaultGamemode {}
