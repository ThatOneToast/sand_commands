use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ItemState {
    pub enchantments: Option<EnchantmentLevels>,
    pub potion_contents: Option<PotionContents>,
    pub can_place_on: Option<Vec<String>>,
    pub can_break: Option<Vec<String>>,
    pub max_stack_size: Option<u32>,
    pub max_damage: Option<u32>,
    pub damage: Option<u32>,
    pub tool: Option<ToolProperties>,
    pub display: Option<ItemDisplay>,
}

#[derive(Debug, Clone)]
pub struct LoreItem {
    pub content: String,
    pub color: Option<String>,
    pub italic: bool,
}

impl Default for LoreItem {
    fn default() -> LoreItem {
        LoreItem {
            content: "PlaceHolder".to_string(),
            color: None,
            italic: true,
        }
    }
}

impl LoreItem {
    pub fn basic(str: &str) -> LoreItem {
        LoreItem {
            content: str.to_string(),
            color: None,
            italic: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameItem {
    pub content: String,
    pub color: Option<String>,
    pub bold: bool,
}

impl Default for NameItem {
    fn default() -> NameItem {
        NameItem {
            content: "PlaceHolder".to_string(),
            color: None,
            bold: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemDisplay {
    pub lore: Option<Vec<LoreItem>>,
    pub name: Option<NameItem>,
}

impl Default for ItemDisplay {
    fn default() -> Self {
        ItemDisplay {
            lore: None,
            name: None,
        }
    }
}

impl ToString for ItemDisplay {
    fn to_string(&self) -> String {
        let mut string = "display:".to_string();
        let mut parts: Vec<String> = Vec::new();

        if self.lore.is_some() {
            let mut lore_strings = Vec::<String>::new();
            let lores = self.lore.as_ref().unwrap();
            for lore_item in lores {
                let lore = lore_item.content.to_owned();
                let color = lore_item.color.to_owned().unwrap_or("magenta".to_string());
                let itallic_use = lore_item.italic;

                lore_strings.push(format!("{{ \"text\": \"{lore}\", \"color\": \"{color}\", \"italic\": {itallic_use} }}").to_string());
            }
            parts.push(format!("Lore:[{}]", lore_strings.join(",")));
        }

        if self.name.is_some() {
            let name_item = self.name.as_ref().unwrap();
            let name = name_item.content.to_owned();
            let color = name_item.color.to_owned().unwrap_or("white".to_string());
            let bold = name_item.bold;

            let name_string = format!(
                "{{\"Name\":{{\"text\": \"{name}\", \"color\": \"{color}\", \"bold\": {bold} }} }}"
            );
            parts.push(format!("{}", name_string));
        }

        let final_part = parts.join(",");
        string.push_str(final_part.as_str());

        string
    }
}

#[derive(Debug, Clone)]
pub struct EnchantmentLevels {
    pub levels: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub struct PotionContents {
    pub potion: Option<String>,              // For basic potions
    pub custom_effects: Option<Vec<Effect>>, // For custom effects
}

#[derive(Debug, Clone)]
pub struct Effect {
    pub id: String,
    pub amplifier: i32,
    pub duration: i32,
}

#[derive(Debug, Clone)]
pub struct ToolProperties {
    pub default_mining_speed: f32,
    pub damage_per_block: u32,
    pub rules: Vec<MiningRule>,
}

#[derive(Debug, Clone)]
pub struct MiningRule {
    pub blocks: String, // Can be specific block or tag like "#mineable/pickaxe"
    pub speed: f32,
    pub correct_for_drops: bool,
}

impl ToString for ItemState {
    fn to_string(&self) -> String {
        let mut states = Vec::new();

        // Always add attributes in a consistent order
        if let Some(enchants) = &self.enchantments {
            let mut sorted_enchants: Vec<_> = enchants.levels.iter().collect();
            sorted_enchants.sort_by_key(|k| k.0); // Sort by enchantment name
            let enchant_str = sorted_enchants
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", k, v))
                .collect::<Vec<_>>()
                .join(",");
            states.push(format!("enchantments={{levels:{{{}}}}}", enchant_str));
        }

        if let Some(potion) = &self.potion_contents {
            if let Some(basic_potion) = &potion.potion {
                states.push(format!("minecraft:potion_contents=\"{}\"", basic_potion));
            }
            if let Some(effects) = &potion.custom_effects {
                let effects_str = effects
                    .iter()
                    .map(|e| {
                        format!(
                            "{{id:\"{}\",amplifier:{},duration:{}}}",
                            e.id, e.amplifier, e.duration
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                states.push(format!(
                    "potion_contents={{custom_effects:[{}]}}",
                    effects_str
                ));
            }
        }

        if let Some(place_on) = &self.can_place_on {
            let mut sorted_blocks = place_on.clone();
            sorted_blocks.sort();
            let blocks_array = sorted_blocks
                .iter()
                .map(|block| format!("\"{}\"", block))
                .collect::<Vec<_>>()
                .join(",");
            states.push(format!("minecraft:can_place_on={{blocks:[{blocks_array}]}}"))
        }

        if let Some(can_break) = &self.can_break {
            let mut sorted_blocks = can_break.clone();
            sorted_blocks.sort();
            let blocks_array = sorted_blocks
                .iter()
                .map(|block| format!("\"{}\"", block))
                .collect::<Vec<_>>()
                .join(",");
            states.push(format!("minecraft:can_break={{blocks:[{blocks_array}]}}"))
        }

        if let Some(max_damage) = &self.max_damage {
            states.push(format!("max_damage={}", max_damage));
        }

        if let Some(damage) = &self.damage {
            states.push(format!("damage={}", damage));
        }

        if let Some(max_stack) = &self.max_stack_size {
            states.push(format!("max_stack_size={}", max_stack));
        }

        if let Some(tool) = &self.tool {
            let mut rules = tool
                .rules
                .iter()
                .map(|r| {
                    format!(
                        "{{blocks:\"{}\",speed:{},correct_for_drops:{}}}",
                        r.blocks, r.speed, r.correct_for_drops
                    )
                })
                .collect::<Vec<_>>();
            rules.sort(); // Sort rules for consistency
            states.push(format!(
                "tool={{default_mining_speed:{},damage_per_block:{},rules:[{}]}}",
                tool.default_mining_speed,
                tool.damage_per_block,
                rules.join(",")
            ));
        }

        // Sort all states for consistent ordering
        states.sort();

        if states.is_empty() {
            String::new()
        } else {
            format!("[{}]", states.join(","))
        }
    }
}
