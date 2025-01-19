use super::MinecraftCommand;

#[derive(Debug, Clone)]
pub struct Damage {
    pub target: crate::types::selector::TargetSelector,
    pub damage: crate::types::combat::DamageCommand,
}

impl Damage {
    pub fn new(
        target: crate::types::selector::TargetSelector,
        damage: crate::types::combat::DamageCommand,
    ) -> Self {
        Self { target, damage }
    }
}

impl ToString for Damage {
    fn to_string(&self) -> String {
        let mut command = format!("damage {} {}", self.target.to_string(), self.damage.amount);

        if let Some(damage_type) = &self.damage.damage_type {
            command.push_str(&format!(" {}", damage_type.to_string()));
        }

        let mut flags = Vec::new();
        if self.damage.at_full_damage {
            flags.push("at_full_damage");
        }
        if self.damage.bypass_armor {
            flags.push("bypass_armor");
        }
        if self.damage.bypass_invulnerability {
            flags.push("bypass_invulnerability");
        }
        if self.damage.bypass_magic {
            flags.push("bypass_magic");
        }
        if self.damage.bypass_absorption {
            flags.push("bypass_absorption");
        }
        if self.damage.bypass_enchantments {
            flags.push("bypass_enchantments");
        }

        if !flags.is_empty() {
            command.push_str(&format!(" {}", flags.join(" ")));
        }

        command
    }
}

impl MinecraftCommand for Damage {}

