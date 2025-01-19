use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Enchant {
    pub target: crate::types::selector::TargetSelector,
    pub enchantment: crate::enchantments::Enchantments,
}

impl Enchant {
    pub fn new(
        enchantment: crate::enchantments::Enchantments,
        target: crate::types::selector::TargetSelector,
    ) -> Self {
        Self {
            target,
            enchantment,
        }
    }
}

impl ToString for Enchant {
    fn to_string(&self) -> String {
        let mut string = String::new();
        string.push_str("enchant ");
        string.push_str(&self.target.to_string());
        string.push_str(" ");
        let (name, level) = self.enchantment.to_pair();
        string.push_str(name.as_str());
        string.push_str(format!(" {}", level).as_str());
        string
    }
}

impl MinecraftCommand for Enchant {}
