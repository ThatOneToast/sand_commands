use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Gamemode(
    crate::types::world::Gamemode,
    crate::types::selector::TargetSelector,
);

impl Gamemode {
    pub fn new(
        gamemode: crate::types::world::Gamemode,
        selector: crate::types::selector::TargetSelector,
    ) -> Self {
        Self(gamemode, selector)
    }
}

impl ToString for Gamemode {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let gamemode = self.0.to_string();
        let selector = self.1.to_string();
        string.push_str("gamemode ");
        string.push_str(&gamemode);
        string.push_str(" ");
        string.push_str(&selector);
        string
    }
}

impl MinecraftCommand for Gamemode {}
