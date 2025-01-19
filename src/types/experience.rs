#[derive(Debug, Clone, Copy)]
pub enum EXP {
    Levels(u32),
    Points(u32),
}

impl EXP {
    pub fn type_(&self) -> String {
        match self {
            EXP::Levels(_) => "levels".to_string(),
            EXP::Points(_) => "points".to_string(),
        }
    }
}

impl ToString for EXP {
    fn to_string(&self) -> String {
        match self {
            EXP::Levels(levels) => format!("{} levels", levels),
            EXP::Points(points) => format!("{} points", points),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EXPAction {
    Add {
        selector: super::selector::TargetSelector,
        etype: EXP,
    },
    Set {
        selector: super::selector::TargetSelector,
        etype: EXP,
    },
    Query {
        selector: super::selector::TargetSelector,
        etype: EXP,
    },
}

impl ToString for EXPAction {
    fn to_string(&self) -> String {
        match self {
            EXPAction::Set { selector, etype } => {
                format!("set {} {}", selector.to_string(), etype.to_string())
            }
            EXPAction::Query { selector, etype } => {
                format!("query {} {}", selector.to_string(), etype.type_())
            }
            EXPAction::Add { selector, etype } => {
                format!("add {} {}", selector.to_string(), etype.to_string())
            }
        }
    }
}
