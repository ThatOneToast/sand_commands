
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Enchantments {
    Protection(u32),
    FireProtection(u32),
    FeatherFalling(u32),
    BlastProtection(u32),
    ProjectileProtection(u32),
    Respiration(u32),
    AquaAffinity(u32),
    Thorns(u32),
    DepthStrider(u32),
    FrostWalker(u32),
    BindingCurse(u32),
    Sharpness(u32),
    Smite(u32),
    BaneOfArthropods(u32),
    Knockback(u32),
    FireAspect(u32),
    Looting(u32),
    Sweeping(u32),
    Efficiency(u32),
    SilkTouch(u32),
    Unbreaking(u32),
    Fortune(u32),
    Power(u32),
    Punch(u32),
    Flame(u32),
    Infinity(u32),
    LuckOfTheSea(u32),
    Lure(u32),
    Mending(u32),
    VanishingCurse(u32),
    SoulSpeed(u32),
    SwiftSneak(u32),
    Other(String, u32), // String for custom enchantment name, u32 for level
}

impl Enchantments {
    pub fn to_pair(&self) -> (String, u32) {
        match self {
            Enchantments::Protection(level) => ("minecraft:protection".to_string(), *level),
            Enchantments::FireProtection(level) => ("minecraft:fire_protection".to_string(), *level),
            Enchantments::FeatherFalling(level) => ("minecraft:feather_falling".to_string(), *level),
            Enchantments::BlastProtection(level) => {
                ("minecraft:blast_protection".to_string(), *level)
            }
            Enchantments::ProjectileProtection(level) => {
                ("minecraft:projectile_protection".to_string(), *level)
            }
            Enchantments::Respiration(level) => ("minecraft:respiration".to_string(), *level),
            Enchantments::AquaAffinity(level) => ("minecraft:aqua_affinity".to_string(), *level),
            Enchantments::Thorns(level) => ("minecraft:thorns".to_string(), *level),
            Enchantments::DepthStrider(level) => ("minecraft:depth_strider".to_string(), *level),
            Enchantments::FrostWalker(level) => ("minecraft:frost_walker".to_string(), *level),
            Enchantments::BindingCurse(level) => ("minecraft:binding_curse".to_string(), *level),
            Enchantments::Sharpness(level) => ("minecraft:sharpness".to_string(), *level),
            Enchantments::Smite(level) => ("minecraft:smite".to_string(), *level),
            Enchantments::BaneOfArthropods(level) => {
                ("minecraft:bane_of_arthropods".to_string(), *level)
            }
            Enchantments::Knockback(level) => ("minecraft:knockback".to_string(), *level),
            Enchantments::FireAspect(level) => ("minecraft:fire_aspect".to_string(), *level),
            Enchantments::Looting(level) => ("minecraft:looting".to_string(), *level),
            Enchantments::Sweeping(level) => ("minecraft:sweeping".to_string(), *level),
            Enchantments::Efficiency(level) => ("minecraft:efficiency".to_string(), *level),
            Enchantments::SilkTouch(level) => ("minecraft:silk_touch".to_string(), *level),
            Enchantments::Unbreaking(level) => ("minecraft:unbreaking".to_string(), *level),
            Enchantments::Fortune(level) => ("minecraft:fortune".to_string(), *level),
            Enchantments::Power(level) => ("minecraft:power".to_string(), *level),
            Enchantments::Punch(level) => ("minecraft:punch".to_string(), *level),
            Enchantments::Flame(level) => ("minecraft:flame".to_string(), *level),
            Enchantments::Infinity(level) => ("minecraft:infinity".to_string(), *level),
            Enchantments::LuckOfTheSea(level) => ("minecraft:luck_of_the_sea".to_string(), *level),
            Enchantments::Lure(level) => ("minecraft:lure".to_string(), *level),
            Enchantments::Mending(level) => ("minecraft:mending".to_string(), *level),
            Enchantments::VanishingCurse(level) => ("minecraft:vanishing_curse".to_string(), *level),
            Enchantments::SoulSpeed(level) => ("minecraft:soul_speed".to_string(), *level),
            Enchantments::SwiftSneak(level) => ("minecraft:swift_sneak".to_string(), *level),
            Enchantments::Other(name, level) => {
                (format!("minecraft:{}", name.to_lowercase()), *level)
            }
        }
    }
}
