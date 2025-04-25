pub mod block_entity_format;
pub mod entity_format;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Looking {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    UP,
    DOWN,
}

impl Looking {
    pub fn to_value(&self) -> u8 {
        match self {
            Self::NORTH => 2,
            Self::EAST => 5,
            Self::SOUTH => 3,
            Self::WEST => 4,
            Self::DOWN => 0,
            Self::UP => 1,
        }
    }
    
    pub fn from_value(value: impl Into<u8>) -> Option<Self> {
        match value.into() {
            2 => Some(Self::NORTH),
            5 => Some(Self::EAST),
            3 => Some(Self::SOUTH),
            4 => Some(Self::WEST),
            0 => Some(Self::DOWN),
            1 => Some(Self::UP),
            _ => None,
        }
    }
}
