
pub mod nbt;
pub mod attribute_modifiers;
pub mod banner_patterns;
pub mod base_color;
pub mod bees;

pub enum DataComponents {
    AttributeModifiers(attribute_modifiers::AttributeModifiers),
    BannerPatterns(banner_patterns::BannerPatterns),
    BaseColor(base_color::BaseColor),
    Bees(bees::Bees),
}


#[derive(Clone, Debug)]
pub enum ComponentSlot {
    ANY,
    HAND,
    ARMOR,
    MainHand,
    OffHand,
    HEAD,
    CHEST,
    LEGS,
    FEET,
    BODY,
}

impl ToString for ComponentSlot {
    fn to_string(&self) -> String {
        match self {
            ComponentSlot::ANY => String::from("any"),
            ComponentSlot::HAND => String::from("hand"),
            ComponentSlot::ARMOR => String::from("armor"),
            ComponentSlot::MainHand => String::from("mainhand"),
            ComponentSlot::OffHand => String::from("offhand"),
            ComponentSlot::HEAD => String::from("head"),
            ComponentSlot::CHEST => String::from("chest"),
            ComponentSlot::LEGS => String::from("legs"),
            ComponentSlot::FEET => String::from("feet"),
            ComponentSlot::BODY => String::from("body"),
        }
    }
}

impl Default for ComponentSlot {
    fn default() -> Self {
        Self::ANY
    }
}
