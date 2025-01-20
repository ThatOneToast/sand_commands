#[derive(Debug, Clone)]
pub enum TagAction {
    Add(String),    // tag <targets> add <name>
    Remove(String), // tag <targets> remove <name>
    List,           // tag <targets> list
}

#[derive(Debug, Clone)]
pub enum MinecraftType {
    BYTE,
    SHORT,
    INT,
    LONG,
    FLOAT,
    DOUBLE,
}

impl ToString for MinecraftType {
    fn to_string(&self) -> String {
        match self {
            Self::BYTE => "byte".to_string(),
            Self::SHORT => "short".to_string(),
            Self::INT => "int".to_string(),
            Self::LONG => "long".to_string(),
            Self::FLOAT => "float".to_string(),
            Self::DOUBLE => "double".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MinecraftComparative {
    LessThan,
    LessThanOrEqual,
    EQUAL,
    GreaterThanOrEqual,
    GreaterThan,
}

impl ToString for MinecraftComparative {
    fn to_string(&self) -> String {
        match self {
            MinecraftComparative::LessThan => "<".to_string(),
            MinecraftComparative::LessThanOrEqual => "<=".to_string(),
            MinecraftComparative::EQUAL => "=".to_string(),
            MinecraftComparative::GreaterThanOrEqual => ">=".to_string(),
            MinecraftComparative::GreaterThan => ">".to_string(),
        }
    }
}
