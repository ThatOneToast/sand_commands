use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub enum SaveAction {
    ON,
    OFF,
    ALL,
}

#[derive(Debug, Clone)]
pub struct Save(SaveAction);

impl Save {
    pub fn new(sa: SaveAction) -> Save {
        Save(sa)
    }
}

impl ToString for Save {
    fn to_string(&self) -> String {
        match self.0 {
            SaveAction::ON => format!("/save-on"),
            SaveAction::OFF => format!("/save-off"),
            SaveAction::ALL => format!("/save-all"),
        }
    }
}

impl MinecraftCommand for Save {}
