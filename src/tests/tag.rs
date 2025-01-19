use crate::{assert_validation, prelude::MinecraftEntity};

#[tokio::test]
    async fn add_tag_self() {
        let command = crate::commands::Tag::new(
            crate::types::selector::TargetSelector::Current(None),
            crate::types::data::TagAction::Add("test_tag".to_string()),
        );
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn add_tag_filtered() {
        let command = crate::commands::Tag::new(
            crate::types::selector::TargetSelector::All(Some(crate::target_filter!(@MinecraftEntity, distance = "..10", team = "Red"))),
            crate::types::data::TagAction::Add("team_tag".to_string()),
        );
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn remove_tag() {
        let command = crate::commands::Tag::new(
            crate::types::selector::TargetSelector::Current(None),
            crate::types::data::TagAction::Remove("test_tag".to_string()),
        );
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn list_tags() {
        let command = crate::commands::Tag::new(
            crate::types::selector::TargetSelector::Current(None),
            crate::types::data::TagAction::List,
        );
        assert_validation!(command.to_string(), true);
    }