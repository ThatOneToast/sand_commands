#[derive(Debug, Clone)]
pub enum DamageType {
    Explosion,
    Fire,
    FireTick,
    Lava,
    Lightning,
    Magic,
    Poison,
    Thorns,
    Trident,
    Wither,
    Anvil,
    Falling,
    DragonBreath,
    Dryout,
    Freeze,
    Stalagmite,
    Starve,
    Void,
    Generic,
}

impl ToString for DamageType {
    fn to_string(&self) -> String {
        match self {
            DamageType::Explosion => "minecraft:explosion".to_string(),
            DamageType::Fire => "minecraft:fire".to_string(),
            DamageType::FireTick => "minecraft:fire_tick".to_string(),
            DamageType::Lava => "minecraft:lava".to_string(),
            DamageType::Lightning => "minecraft:lightning".to_string(),
            DamageType::Magic => "minecraft:magic".to_string(),
            DamageType::Poison => "minecraft:poison".to_string(),
            DamageType::Thorns => "minecraft:thorns".to_string(),
            DamageType::Trident => "minecraft:trident".to_string(),
            DamageType::Wither => "minecraft:wither".to_string(),
            DamageType::Anvil => "minecraft:anvil".to_string(),
            DamageType::Falling => "minecraft:falling".to_string(),
            DamageType::DragonBreath => "minecraft:dragon_breath".to_string(),
            DamageType::Dryout => "minecraft:dryout".to_string(),
            DamageType::Freeze => "minecraft:freeze".to_string(),
            DamageType::Stalagmite => "minecraft:stalagmite".to_string(),
            DamageType::Starve => "minecraft:starve".to_string(),
            DamageType::Void => "minecraft:void".to_string(),
            DamageType::Generic => "minecraft:generic".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DamageCommand {
    pub amount: f32,
    pub damage_type: Option<DamageType>,
    pub at_full_damage: bool,
    pub bypass_armor: bool,
    pub bypass_invulnerability: bool,
    pub bypass_magic: bool,
    pub bypass_absorption: bool,
    pub bypass_enchantments: bool,
}

impl Default for DamageCommand {
    fn default() -> Self {
        Self {
            amount: 0.0,
            damage_type: None,
            at_full_damage: false,
            bypass_armor: false,
            bypass_invulnerability: false,
            bypass_magic: false,
            bypass_absorption: false,
            bypass_enchantments: false,
        }
    }
}
