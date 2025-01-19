use crate::types::OPAction;

use super::MinecraftCommand;



#[derive(Debug, Clone)]
pub struct OP(OPAction);

impl OP {
    pub fn new(action: OPAction) -> Self {
        OP(action)
    }
}

impl ToString for OP {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl MinecraftCommand for OP {}
