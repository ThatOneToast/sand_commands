use crate::assert_validation;

#[tokio::test]
async fn basic_trigger() {
    let command = crate::commands::Trigger::new("points".to_string(), None);
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn trigger_add() {
    let command = crate::commands::Trigger::new(
        "score".to_string(),
        Some(crate::types::scoreboard::TriggerAction::Add(5)),
    );
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn trigger_set() {
    let command = crate::commands::Trigger::new(
        "level".to_string(),
        Some(crate::types::scoreboard::TriggerAction::Set(10)),
    );
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn trigger_negative() {
    let command = crate::commands::Trigger::new(
        "level".to_string(),
        Some(crate::types::scoreboard::TriggerAction::Set(-1)),
    );
    assert_validation!(command.to_string(), true);
}
