use serde::{Deserialize, Serialize};

use super::ComponentSlot;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttributeModifier {
    pub attribute_type: Attribute,
    pub slot: ComponentSlot,
    pub id: String,
    pub amount: u64,
    pub operation: ModifierOperation,
}

impl ToString for AttributeModifier {
    fn to_string(&self) -> String {
        let type_ = format!("type:\"{}\"", self.attribute_type.to_string());
        let slot = format!("slot:\"{}\"", self.slot.to_string());
        let id = format!("id:\"{}\"", self.id);
        let amount = format!("amount:{}", self.amount);
        let operation = format!("operation:\"{}\"", self.operation.to_string());
        format!("{{{type_},{slot},{id},{amount},{operation}}}")
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttributeModifiers(pub Vec<AttributeModifier>);

impl ToString for AttributeModifiers {
    fn to_string(&self) -> String {
        let modifiers = self
            .0
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",");
        format!("attribute_modifiers=[{modifiers}]")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Attribute {
    MaxHealth,
    MaxAbsorption,
    FollowRange,
    KnockbackResistance,
    MovementSpeed,
    AttackDamage,
    ARMOR,
    ArmorToughness,
    AttackKnockback,
    AttackSpeed,
    LUCK,
    JumpStrength,
    FlyingSpeed,
    SpawnReinforcements,
    // 1.21+ additions:
    GRAVITY,
    SafeFallDistance,
    FallDamageMultiplier,
    BlockBreakSpeed,
    BurningTime,
    ExplosionKnockbackResistance,
    MiningEfficiency,
    MovementEfficiency,
    OxygenBonus,
    SneakingSpeed,
    SubmergedMiningSpeed,
    SweepingDamageRatio,
    WaterMovementEfficiency,
    TemptRange,
    CameraDistance,
    WaypointTransmitRange,
    WaypointReceiveRange,
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        let s = match self {
            Attribute::MaxHealth => "max_health",
            Attribute::MaxAbsorption => "max_absorption",
            Attribute::FollowRange => "follow_range",
            Attribute::KnockbackResistance => "knockback_resistance",
            Attribute::MovementSpeed => "movement_speed",
            Attribute::AttackDamage => "attack_damage",
            Attribute::ARMOR => "armor",
            Attribute::ArmorToughness => "armor_toughness",
            Attribute::AttackKnockback => "attack_knockback",
            Attribute::AttackSpeed => "attack_speed",
            Attribute::LUCK => "luck",
            Attribute::JumpStrength => "jump_strength",
            Attribute::FlyingSpeed => "flying_speed",
            Attribute::SpawnReinforcements => "spawn_reinforcements",
            Attribute::GRAVITY => "gravity",
            Attribute::SafeFallDistance => "safe_fall_distance",
            Attribute::FallDamageMultiplier => "fall_damage_multiplier",
            Attribute::BlockBreakSpeed => "block_break_speed",
            Attribute::BurningTime => "burning_time",
            Attribute::ExplosionKnockbackResistance => "explosion_knockback_resistance",
            Attribute::MiningEfficiency => "mining_efficiency",
            Attribute::MovementEfficiency => "movement_efficiency",
            Attribute::OxygenBonus => "oxygen_bonus",
            Attribute::SneakingSpeed => "sneaking_speed",
            Attribute::SubmergedMiningSpeed => "submerged_mining_speed",
            Attribute::SweepingDamageRatio => "sweeping_damage_ratio",
            Attribute::WaterMovementEfficiency => "water_movement_efficiency",
            Attribute::TemptRange => "tempt_range",
            Attribute::CameraDistance => "camera_distance",
            Attribute::WaypointTransmitRange => "waypoint_transmit_range",
            Attribute::WaypointReceiveRange => "waypoint_receive_range",
        };
        s.to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModifierOperation {
    AddValue,
    AddMultipliedBase,
    AddMultipliedTotal,
}

impl ToString for ModifierOperation {
    fn to_string(&self) -> String {
        match self {
            ModifierOperation::AddValue => String::from("add_value"),
            ModifierOperation::AddMultipliedBase => String::from("add_multiplied_base"),
            ModifierOperation::AddMultipliedTotal => String::from("add_multiplied_total"),
        }
    }
}
