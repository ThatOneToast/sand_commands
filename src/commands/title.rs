use crate::types::text::TitleAction;

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Title {
    pub target: crate::types::selector::TargetSelector,
    pub action: crate::types::text::TitleAction,
}

impl Title {
    pub fn new(target: crate::types::selector::TargetSelector, action: crate::types::text::TitleAction) -> Self {
        Self { target, action }
    }
}

impl ToString for Title {
    fn to_string(&self) -> String {
        match &self.action {
            TitleAction::Title(text) => format!("title {} title \"{}\"", self.target.to_string(), text),
            TitleAction::Subtitle(text) => format!("title {} subtitle \"{}\"", self.target.to_string(), text),
            TitleAction::Actionbar(text) => format!("title {} actionbar \"{}\"", self.target.to_string(), text),
            TitleAction::Clear => format!("title {} clear", self.target.to_string()),
            TitleAction::Reset => format!("title {} reset", self.target.to_string()),
            TitleAction::Times { fade_in, stay, fade_out } => {
                format!("title {} times {} {} {}", self.target.to_string(), fade_in, stay, fade_out)
            }
        }
    }
}

impl MinecraftCommand for Title {}
