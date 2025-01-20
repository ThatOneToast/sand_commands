use std::any::Any;

pub use ban::Ban;
pub use damage::Damage;
pub use defaultgamemode::DefaultGamemode;
pub use difficulty::Difficulty;
pub use enchant::Enchant;
pub use execute::Execute;
pub use fill::Fill;
pub use gamemode::Gamemode;
pub use gamerule::GameRule;
pub use give::Give;
pub use kill::Kill;
pub use op::OP;
pub use save::{Save, SaveAction};
pub use scoreboard::Scoreboard;
pub use setblock::Setblock;
pub use tag::Tag;
pub use teleport::Teleport;
pub use time::Time;
pub use title::Title;
pub use trigger::Trigger;
pub use unban::UnBan;
pub use weather::Weather;
pub use whitelist::Whitelist;
pub use world_border::WorldBorder;
pub use xp::XP;

pub mod ban;
pub mod damage;
pub mod defaultgamemode;
pub mod difficulty;
pub mod enchant;
pub mod execute;
pub mod fill;
pub mod gamemode;
pub mod gamerule;
pub mod give;
pub mod kill;
pub mod op;
pub mod save;
pub mod scoreboard;
pub mod setblock;
pub mod tag;
pub mod team;
pub mod teleport;
pub mod time;
pub mod title;
pub mod trigger;
pub mod unban;
pub mod weather;
pub mod whitelist;
pub mod world_border;
pub mod xp;

#[derive(Debug, Clone)]
pub enum DefaultCommands {
    BAN(Ban),
    DAMAGE(Damage),
    DEFAULTGAMEMODE(DefaultGamemode),
    DIFFICULTY(Difficulty),
    ENCHANT(Enchant),
    FILL(Fill),
    GAMEMODE(Gamemode),
    GAMERULE(GameRule),
    GIVE(Give),
    KILL(Kill),
    OP(OP),
    SCOREBOARD(Scoreboard),
    SETBLOCK(Setblock),
    TAG(Tag),
    TELEPORT(Teleport),
    TIME(Time),
    TITLE(Title),
    TRIGGER(Trigger),
    UNBAN(UnBan),
    WEATHER(Weather),
    WHITELIST(Whitelist),
    WORLDBORDER(WorldBorder),
    XP(XP),
    SAVE(Save),
    EXECUTE(Execute),
}

impl DefaultCommands {
    pub fn inner(&self) -> Box<dyn MinecraftCommand> {
        match self.to_owned() {
            DefaultCommands::BAN(command) => Box::new(command.clone()),
            DefaultCommands::DAMAGE(command) => Box::new(command.clone()),
            DefaultCommands::DEFAULTGAMEMODE(command) => Box::new(command.clone()),
            DefaultCommands::DIFFICULTY(command) => Box::new(command.clone()),
            DefaultCommands::ENCHANT(command) => Box::new(command.clone()),
            DefaultCommands::FILL(command) => Box::new(command.clone()),
            DefaultCommands::GAMEMODE(command) => Box::new(command.clone()),
            DefaultCommands::GAMERULE(command) => Box::new(command.clone()),
            DefaultCommands::GIVE(command) => Box::new(command.clone()),
            DefaultCommands::KILL(command) => Box::new(command.clone()),
            DefaultCommands::OP(command) => Box::new(command.clone()),
            DefaultCommands::SCOREBOARD(command) => Box::new(command.clone()),
            DefaultCommands::SETBLOCK(command) => Box::new(command.clone()),
            DefaultCommands::TAG(command) => Box::new(command.clone()),
            DefaultCommands::TELEPORT(command) => Box::new(command.clone()),
            DefaultCommands::TIME(command) => Box::new(command.clone()),
            DefaultCommands::TITLE(command) => Box::new(command.clone()),
            DefaultCommands::TRIGGER(command) => Box::new(command.clone()),
            DefaultCommands::UNBAN(command) => Box::new(command.clone()),
            DefaultCommands::WEATHER(command) => Box::new(command.clone()),
            DefaultCommands::WHITELIST(command) => Box::new(command.clone()),
            DefaultCommands::WORLDBORDER(command) => Box::new(command.clone()),
            DefaultCommands::XP(command) => Box::new(command.clone()),
            DefaultCommands::SAVE(command) => Box::new(command.clone()),
            DefaultCommands::EXECUTE(command) => Box::new(command.clone()),
        }
    }
}

impl ToString for DefaultCommands {
    fn to_string(&self) -> String {
        self.inner().to_string()
    }
}

impl MinecraftCommandCollection for DefaultCommands {}

pub trait MinecraftCommand: ToString + std::fmt::Debug + Any {}
pub trait MinecraftCommandCollection: ToString + std::fmt::Debug + Clone + Any {}
