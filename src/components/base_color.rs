use super::Component;


#[derive(Clone, Debug)]
pub struct BaseColor {
    pub color: String,
}

impl ToString for BaseColor {
    fn to_string(&self) -> String {
        let color = &self.color;
        format!("base_color=\"{color}\"")
    }
}

impl Component for BaseColor {}
