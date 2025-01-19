use crate::{assert_validation, commands::Teleport, prelude::MinecraftEntity, target_filter, types::{Location, TargetSelector, TeleportTargetLocation}};


#[tokio::test]
async fn self_to_coord() {
    let command = Teleport {
        teleporting_self: true,
        target: None,
        location: TeleportTargetLocation {
            pos: Some(Location {
                x: 100.0,
                y: 64.0,
                z: -100.0,
            }),
            entity: None,
        },
    };
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn player_to_coords() {
    let command = Teleport {
        teleporting_self: false,
        target: Some(TargetSelector::Other("Player123".to_string())),
        location: TeleportTargetLocation {
            pos: Some(Location {
                x: 100.0,
                y: 64.0,
                z: -100.0,
            }),
            entity: None,
        },
    };
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn filtered_player_to_coords() {
    let filter = target_filter!(@MinecraftEntity, distance = "10..", team = "Red", level = "..50");
    let command = Teleport {
        teleporting_self: false,
        target: Some(TargetSelector::All(Some(filter))),
        location: TeleportTargetLocation {
            pos: Some(Location {
                x: 100.0,
                y: 64.0,
                z: -100.0,
            }),
            entity: None,
        },
    };
    
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn filtered_players_to_coords() {
    let filter = target_filter!(@MinecraftEntity, level = "30..", distance = "10..", team = "Red");
    let command = Teleport {
        teleporting_self: false,
        target: Some(TargetSelector::All(Some(filter))),
        location: TeleportTargetLocation {
            pos: Some(Location {
                x: 100.0,
                y: 64.0,
                z: -100.0,
            }),
            entity: None,
        },
    };
    
    assert_validation!(command.to_string(), true);
}