use crate::assert_validation;

#[tokio::test]
async fn add_player() {
    let command = crate::commands::whitelist::Whitelist::new(
        crate::types::permissions::WhitelistAction::Add("Player1".to_string()),
    );
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn add_player_special() {
    let command = crate::commands::whitelist::Whitelist::new(
        crate::types::permissions::WhitelistAction::Add("Player_123".to_string()),
    );
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn remove_player() {
    let command = crate::commands::whitelist::Whitelist::new(
        crate::types::permissions::WhitelistAction::Remove("Player1".to_string()),
    );
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn toggle_whitelist() {
    let command =
        crate::commands::whitelist::Whitelist::new(crate::types::permissions::WhitelistAction::On);
    assert_validation!(command.to_string(), true);
}
