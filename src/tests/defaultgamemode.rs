use crate::{assert_validation, prelude::defaultgamemode::DefaultGamemode};


#[tokio::test]
async fn creative() {
    let command = DefaultGamemode::new(crate::types::Gamemode::Creative);
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn survival() {
    let command = DefaultGamemode::new(crate::types::Gamemode::Survival);
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn adventure() {
    let command = DefaultGamemode::new(crate::types::Gamemode::Adventure);
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn spectator() {
    let command = DefaultGamemode::new(crate::types::Gamemode::Spectator);
    assert_validation!(command.to_string(), true);
}