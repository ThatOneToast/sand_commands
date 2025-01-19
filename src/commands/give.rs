use crate::types::ItemState;

use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Give {
    pub target: crate::types::selector::TargetSelector,
    pub item: String,
    pub block_states: Option<ItemState>,
    pub count: Option<u32>,
}

impl Give {
    pub fn new(
        target: crate::types::selector::TargetSelector,
        item: String,
        block_states: Option<ItemState>,
        count: Option<u32>,
    ) -> Self {
        Self {
            target,
            item,
            block_states,
            count,
        }
    }
}

impl ToString for Give {
    fn to_string(&self) -> String {
        let mut command = format!("give {} {}", self.target.to_string(), self.item);

        if let Some(states) = &self.block_states {
            let state_str = states.to_string();
            if !state_str.is_empty() {
                command.push_str(&state_str);
            }
        }

        if let Some(count) = self.count {
            command.push_str(&format!(" {}", count));
        }

        command
    }
}

impl MinecraftCommand for Give {}
