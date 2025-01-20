use crate::{assert_validation, prelude::team::Team, types::{TeamAction, TeamActionModify}};



#[tokio::test]
async fn team_add() {
    let command = Team::new(TeamAction::ADD("test".to_string(), None));
    assert_validation!(command, true)
}

#[tokio::test]
async fn team_remove() {
    let command = Team::new(TeamAction::REMOVE("test".to_string()));
    assert_validation!(command, true)
}

#[tokio::test]
async fn team_empty() {
    let command = Team::new(TeamAction::EMPTY("test".to_string()));
    assert_validation!(command, true)
}

#[tokio::test]
async fn team_join() {
    let command = Team::new(TeamAction::JOIN("test".to_string(), None));
    assert_validation!(command, true)
}

#[tokio::test]
async fn team_leave() {
    let command = Team::new(TeamAction::LEAVE(None));
    assert_validation!(command, true)
}

#[tokio::test]
async fn team_modify() {
    let command = Team::new(TeamAction::MODIFY("test".to_string(), TeamActionModify::DisplayName(crate::types::TextComponent::String("test".to_string()))));
    assert_validation!(command, true)
}

#[tokio::test]
async fn team_list() {
    let command = Team::new(TeamAction::LIST);
    assert_validation!(command, true)
}