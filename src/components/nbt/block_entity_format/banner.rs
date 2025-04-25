use serde::{Deserialize, Serialize};

use crate::components::banner_patterns::BannerPatterns;

use super::BlockEntityBase;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Banner {
    pub base: BlockEntityBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    pub patterns: BannerPatterns,
}

impl ToString for Banner {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize Banner to JSON")
    }
}
