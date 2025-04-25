use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseColor {
    pub color: String,
}

impl ToString for BaseColor {
    fn to_string(&self) -> String {
        let color = &self.color;
        format!("base_color=\"{color}\"")
    }
}

