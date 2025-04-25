use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BannerPatternType {
    FieldMasoned,
    BordureIndented,
    FlowerCharge,
    CreeperCharge,
    SkullCharge,
    Thing,
    Globe,
    Snout,
    FLOW,
    GUSTER,
}

impl ToString for BannerPatternType {
    fn to_string(&self) -> String {
        match self {
            BannerPatternType::FieldMasoned => "field_masoned",
            BannerPatternType::BordureIndented => "bordure_indented",
            BannerPatternType::FlowerCharge => "flower_charge",
            BannerPatternType::CreeperCharge => "creeper_charge",
            BannerPatternType::SkullCharge => "skull_charge",
            BannerPatternType::Thing => "thing",
            BannerPatternType::Globe => "globe",
            BannerPatternType::Snout => "snout",
            BannerPatternType::FLOW => "flow",
            BannerPatternType::GUSTER => "guster",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannerPattern {
    pub pattern: BannerPatternType,
    pub color: String,
}

impl ToString for BannerPattern {
    fn to_string(&self) -> String {
        let pattern = self.pattern.to_string();
        let color = self.color.to_string();
        format!("{{pattern:\"{pattern}\",color:\"{color}\"}}")
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannerPatterns(pub Vec<BannerPattern>);

impl ToString for BannerPatterns {
    fn to_string(&self) -> String {
        let patterns = self
            .0
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(",");
        format!("banner_patterns=[{patterns}]")
    }
}
