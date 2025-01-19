#[derive(Debug, Clone)]
pub enum TitleAction {
    Title(String),
    Subtitle(String),
    Actionbar(String),
    Clear,
    Reset,
    Times {
        fade_in: u32,
        stay: u32,
        fade_out: u32,
    },
}
