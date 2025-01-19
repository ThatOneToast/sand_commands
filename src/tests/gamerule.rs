use crate::assert_validation;

#[tokio::test]
async fn boolean_gamerules() {
    let command =
        crate::commands::GameRule::new(crate::types::world::GameRule::KeepInventory(Some(true)));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn query_boolean_gamerules() {
    let command =
        crate::commands::GameRule::new(crate::types::world::GameRule::DoMobSpawning(None));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn integer_gamerules() {
    let command =
        crate::commands::GameRule::new(crate::types::world::GameRule::RandomTickSpeed(Some(3)));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn modify_gameplay_rules() {
    let command =
        crate::commands::GameRule::new(crate::types::world::GameRule::DoFireTick(Some(false)));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn numeric_bounds() {
    let command =
        crate::commands::GameRule::new(crate::types::world::GameRule::MaxEntityCramming(Some(24)));
    assert_validation!(command.to_string(), true);
}
