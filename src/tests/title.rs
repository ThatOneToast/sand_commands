use crate::{assert_validation, prelude::MinecraftEntity};

#[tokio::test]
    async fn basic_title() {
        let command = crate::commands::Title::new(
            crate::types::selector::TargetSelector::All(None),
            crate::types::text::TitleAction::Title("Welcome!".to_string()),
        );
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn subtitle() {
        let command = crate::commands::Title::new(
            crate::types::selector::TargetSelector::All(None),
            crate::types::text::TitleAction::Subtitle("To the server".to_string()),
        );
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn title_times() {
        let command = crate::commands::Title::new(
            crate::types::selector::TargetSelector::All(None),
            crate::types::text::TitleAction::Times {
                fade_in: 10,
                stay: 70,
                fade_out: 20,
            },
        );
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn filtered_title() {
        let command = crate::commands::Title::new(
            crate::types::selector::TargetSelector::All(Some(crate::target_filter!(@MinecraftEntity, team = "Red", distance = "..10"))),
            crate::types::text::TitleAction::Title("Team Red Wins!".to_string()),
        );
        assert_validation!(command.to_string(), true);
    }