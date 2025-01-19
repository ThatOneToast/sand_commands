
mod objectives {
    use crate::assert_validation;

    

    #[tokio::test]
    async fn add_objective_simple() {
        let command = crate::commands::Scoreboard::new(crate::types::scoreboard::ScoreboardAction::Objectives(
            crate::types::scoreboard::ObjectiveAction::Add {
                objective: "test".to_string(),
                criteria: crate::types::scoreboard::ObjectiveCriteria::Dummy,
                display_name: None,
            }
        ));
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn add_objective_display() {
        let command = crate::commands::Scoreboard::new(crate::types::scoreboard::ScoreboardAction::Objectives(
            crate::types::scoreboard::ObjectiveAction::Add {
                objective: "kills".to_string(),
                criteria: crate::types::scoreboard::ObjectiveCriteria::PlayerKillCount,
                display_name: Some("Player Kills".to_string()),
            }
        ));
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn remove_objective() {
        let command = crate::commands::Scoreboard::new(crate::types::scoreboard::ScoreboardAction::Objectives(
            crate::types::scoreboard::ObjectiveAction::Remove {
                objective: "test".to_string(),
            }
        ));
        assert_validation!(command.to_string(), true);
    }
}

mod players {
    use crate::{assert_validation, prelude::MinecraftEntity};


    #[tokio::test]
    async fn add_score() {
        let command = crate::commands::Scoreboard::new(crate::types::scoreboard::ScoreboardAction::Players(
            crate::types::scoreboard::PlayerAction::Add {
                target: crate::types::selector::TargetSelector::Current(None),
                objective: "score".to_string(),
                score: 5,
            }
        ));
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn set_score() {
        let command = crate::commands::Scoreboard::new(crate::types::scoreboard::ScoreboardAction::Players(
            crate::types::scoreboard::PlayerAction::Set {
                target: crate::types::selector::TargetSelector::All(Some(crate::target_filter!(@MinecraftEntity, team = "Red"))),
                objective: "wins".to_string(),
                score: 10,
            }
        ));
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn reset_score() {
        let command = crate::commands::Scoreboard::new(crate::types::scoreboard::ScoreboardAction::Players(
            crate::types::scoreboard::PlayerAction::Reset {
                target: crate::types::selector::TargetSelector::Current(None),
                objective: Some("deaths".to_string()),
            }
        ));
        assert_validation!(command.to_string(), true);
    }
}