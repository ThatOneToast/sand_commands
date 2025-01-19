use crate::{assert_validation, prelude::ban::BanTarget};


#[tokio::test]
async fn player_ban() {
    let command = crate::commands::Ban::new(BanTarget::Player("toast".to_string()));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn ip_ban() {
    let command = crate::commands::Ban::new(BanTarget::IP("123.456.789.0".to_string()));
    assert_validation!(command.to_string(), true);
}