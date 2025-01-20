use serde::{Deserialize, Serialize};
use std::fmt;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextComponent {
    String(String),
    Array(Vec<TextComponent>),
    Object(TextObject),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ContentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with: Option<Vec<TextComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpret: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<Box<TextComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<TextComponent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Hex(String),
    Named(NamedColor),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickEvent {
    pub action: ClickAction,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    Translatable,
    Score,
    Selector,
    Keybind,
    Nbt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreComponent {
    pub name: String,
    pub objective: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClickAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverEvent {
    pub action: HoverAction,
    pub contents: HoverContents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoverAction {
    ShowText,
    ShowItem,
    ShowEntity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    Text(Box<TextComponent>),
    Item(ShowItemDisplay),
    Entity(EntityDisplay),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowItemDisplay {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDisplay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<TextComponent>>,
    pub type_: String,
    pub id: String,
}

impl fmt::Display for TextComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

impl From<String> for TextComponent {
    fn from(s: String) -> Self {
        TextComponent::String(s)
    }
}

impl<'a> From<&'a str> for TextComponent {
    fn from(s: &'a str) -> Self {
        TextComponent::String(s.to_string())
    }
}
