use std::collections::HashMap;

use crate::{
    assert_validation, bundle_enchants,
    commands::Give,
    prelude::Enchantments,
    types::{
        Effect, ItemDisplay, ItemEnchantments, ItemState, LoreItem, MiningRule, PotionContents, TargetSelector, ToolProperties
    },
};

#[tokio::test]
async fn give_basic() {
    let command = crate::commands::Give::new(
        crate::types::selector::TargetSelector::Current(None),
        "minecraft:diamond_sword".to_string(),
        Some(crate::types::ItemState {
            display: Some(ItemDisplay {
                lore: Some(vec![LoreItem::basic("Lore 1"), LoreItem::basic("Lore 2")]),
                ..Default::default()
            }),
            ..Default::default()
        }),
        Some(1),
    );

    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn ultimate_sword() {
    let enchants = bundle_enchants!(
        Enchantments::Sharpness(5),
        Enchantments::FireAspect(2),
        Enchantments::Looting(3),
        Enchantments::Unbreaking(3),
        Enchantments::Mending(1)
    );

    let command = crate::commands::Give::new(
        crate::types::selector::TargetSelector::Current(None),
        "minecraft:netherite_sword".to_string(),
        Some(ItemState {
            display: Some(ItemDisplay {
                lore: Some(vec![
                    LoreItem::basic("§6Ultimate Weapon"),
                    LoreItem::basic("§7Forged in ancient fires"),
                    LoreItem::basic("§cWields immense power"),
                ]),
                ..Default::default()
            }),
            enchantments: Some(ItemEnchantments(enchants)),
            max_damage: Some(2000),
            damage: Some(10),
            ..Default::default()
        }),
        Some(1),
    );

    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn advanced_sword() {
    let command = Give::new(
        TargetSelector::All(None),
        "minecraft:tipped_arrow".to_string(),
        Some(ItemState {
            potion_contents: Some(PotionContents {
                potion: None,
                custom_effects: Some(vec![
                    Effect {
                        id: "minecraft:slowness".to_string(),
                        amplifier: 2,
                        duration: 200,
                    },
                    Effect {
                        id: "minecraft:poison".to_string(),
                        amplifier: 1,
                        duration: 100,
                    },
                    Effect {
                        id: "minecraft:weakness".to_string(),
                        amplifier: 1,
                        duration: 300,
                    },
                ]),
            }),
            display: Some(ItemDisplay {
                lore: Some(vec![LoreItem::basic("§8Arrows of Misfortune")]),
                ..Default::default()
            }),
            ..Default::default()
        }),
        Some(64),
    );

    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn adventure_mode_pickaxe() {
    let mut tool_enchants = HashMap::new();
    tool_enchants.insert("minecraft:efficiency".to_string(), 5);
    tool_enchants.insert("minecraft:fortune".to_string(), 3);

    let special_pickaxe = Give::new(
        TargetSelector::Current(None),
        "minecraft:netherite_pickaxe".to_string(),
        Some(ItemState {
            enchantments: Some(ItemEnchantments(tool_enchants)),
            can_break: Some(vec![
                "minecraft:diamond_ore".to_string(),
                "minecraft:ancient_debris".to_string(),
                "minecraft:emerald_ore".to_string(),
            ]),
            tool: Some(ToolProperties {
                default_mining_speed: 8.0,
                damage_per_block: 1,
                rules: vec![
                    MiningRule {
                        blocks: "#mineable/pickaxe".to_string(),
                        speed: 12.0,
                        correct_for_drops: true,
                    },
                    MiningRule {
                        blocks: "minecraft:ancient_debris".to_string(),
                        speed: 15.0,
                        correct_for_drops: true,
                    },
                ],
            }),
            max_damage: Some(2031),
            damage: Some(0),
            ..Default::default()
        }),
        Some(1),
    );

    assert_validation!(special_pickaxe.to_string(), true);
}

#[tokio::test]
async fn complex_splash_potion() {
    let mut potion_enchants = HashMap::new();
    potion_enchants.insert("minecraft:unbreaking".to_string(), 3);

    let complex_potion = Give::new(
        TargetSelector::Random(None),
        "minecraft:splash_potion".to_string(),
        Some(ItemState {
            enchantments: Some(ItemEnchantments(potion_enchants)),
            potion_contents: Some(PotionContents {
                potion: None,
                custom_effects: Some(vec![
                    Effect {
                        id: "minecraft:strength".to_string(),
                        amplifier: 3,
                        duration: 1200,
                    },
                    Effect {
                        id: "minecraft:speed".to_string(),
                        amplifier: 2,
                        duration: 1200,
                    },
                    Effect {
                        id: "minecraft:jump_boost".to_string(),
                        amplifier: 2,
                        duration: 1200,
                    },
                    Effect {
                        id: "minecraft:resistance".to_string(),
                        amplifier: 1,
                        duration: 1200,
                    },
                ]),
            }),
            display: Some(ItemDisplay {
                lore: Some(vec![
                    LoreItem::basic("§dPotion of the Warrior"),
                    LoreItem::basic("§7Grants multiple combat buffs"),
                ]),
                ..Default::default()
            }),
            ..Default::default()
        }),
        Some(1),
    );

    assert_validation!(complex_potion.to_string(), true);
}
