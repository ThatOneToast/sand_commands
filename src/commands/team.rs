use crate::types::TeamAction;


#[derive(Debug, Clone)]
pub struct Team(TeamAction);

impl ToString for Team {
    fn to_string(&self) -> String {
        format!("team {}", self.0.to_string())
    }
}

impl Team {
    pub fn new(action: TeamAction) -> Self {
        Self(action)
    }
}