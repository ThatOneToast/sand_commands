use crate::components::banner_patterns::BannerPatterns;

use super::BlockEntityBase;


#[derive(Debug, Clone)]
pub struct Banner {
    pub base: BlockEntityBase,
    pub custom_name: Option<String>,
    pub patterns: BannerPatterns,
}
