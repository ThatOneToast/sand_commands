#[derive(Debug, Clone)]
pub enum TagAction {
    Add(String),    // tag <targets> add <name>
    Remove(String), // tag <targets> remove <name>
    List,           // tag <targets> list
}
