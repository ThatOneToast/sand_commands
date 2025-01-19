use super::MinecraftCommand;




#[derive(Debug, Clone)]
pub struct Difficulty(crate::types::world::Difficulty);

impl Difficulty {
    pub fn new(diff: crate::types::world::Difficulty) -> Self {
        Difficulty(diff)
    }
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        format!("difficulty {}", self.0.to_string())
    }
}

impl MinecraftCommand for Difficulty {}
