use crate::{assert_validation, prelude::ban::BanTarget};



#[tokio::test]
async fn player_unban() {
    let command = crate::commands::UnBan::new(BanTarget::Player("toast".to_string()));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn ip_unban() {
    let command = crate::commands::UnBan::new(BanTarget::IP("123.456.789.0".to_string()));
    assert_validation!(command.to_string(), true);
}