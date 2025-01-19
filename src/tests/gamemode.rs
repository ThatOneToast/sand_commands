use crate::{assert_validation, prelude::MinecraftEntity};




#[tokio::test]
async fn creative() {
    let command = crate::commands::Gamemode::new(
        crate::types::world::Gamemode::Creative,
        crate::types::TargetSelector::Current(None),
    );
    
    assert_validation!(command.to_string(), true);
    
}

#[tokio::test]
async fn survival() {
    let command = crate::commands::Gamemode::new(
        crate::types::world::Gamemode::Survival,
        crate::types::TargetSelector::Current(None),
    );
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn adventure() {
    let command = crate::commands::Gamemode::new(
        crate::types::world::Gamemode::Adventure,
        crate::types::TargetSelector::Current(None),
    );
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn spectator() {
    let command = crate::commands::Gamemode::new(
        crate::types::world::Gamemode::Spectator,
        crate::types::TargetSelector::Current(None),
    );
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn creative_with_distance() {
    let command = crate::commands::Gamemode::new(
        crate::types::world::Gamemode::Creative,
        crate::types::TargetSelector::All(Some(crate::target_filter!(@MinecraftEntity, distance = "10"))),
    );
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn creative_with_team() {
    let command = crate::commands::Gamemode::new(
        crate::types::world::Gamemode::Creative,
        crate::types::TargetSelector::All(Some(crate::target_filter!(@MinecraftEntity, team = "Red"))),
    );
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn creative_with_limit() {
    let command = crate::commands::Gamemode::new(
        crate::types::world::Gamemode::Creative,
        crate::types::TargetSelector::All(Some(crate::target_filter!(@MinecraftEntity, limit = "1"))),
    );
    
    assert_validation!(command.to_string(), true);
}
