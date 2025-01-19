use crate::{assert_validation, types::{DamageCommand, DamageType, TargetSelector}};

#[tokio::test]
async fn basic_damage() {
    let command = crate::commands::Damage::new(
        TargetSelector::Current(None),
        DamageCommand {
            amount: 10.5,
            damage_type: Some(DamageType::Generic),
            ..Default::default()
        },
    );
    
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn damage_with_flags() {
    let command = crate::commands::Damage::new(
        TargetSelector::Current(None),
        DamageCommand {
            amount: 15.0,
            bypass_armor: true,
            bypass_invulnerability: true,
            bypass_magic: true,
            bypass_absorption: true,
            bypass_enchantments: true,
            at_full_damage: true,
            damage_type: Some(DamageType::Magic),
            ..Default::default()
        },
    );
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn damage_with_type_and_flags() {
    let command = crate::commands::Damage::new(
        TargetSelector::Current(None),
        DamageCommand {
            amount: 20.0,
            damage_type: Some(DamageType::Magic),
            bypass_armor: true,
            at_full_damage: true,
            ..Default::default()
        },
    );
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn damage_with_target_selectors() {
    let command = crate::commands::Damage::new(
        TargetSelector::Other("Toast".to_string()),
        DamageCommand {
            amount: 1.0,
            damage_type: Some(DamageType::Magic),
            ..Default::default()
        },
    );
    
    assert_validation!(command.to_string(), true);
}

