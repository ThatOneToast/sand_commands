use crate::assert_validation;

#[tokio::test]
    async fn add_border() {
        let command = crate::commands::WorldBorder::new(crate::types::world::WorldBorderAction::Add {
            distance: 10.0,
            time: None,
        });
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn add_border_with_time() {
        let command = crate::commands::WorldBorder::new(crate::types::world::WorldBorderAction::Add {
            distance: 50.0,
            time: Some(100),
        });
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn set_center() {
        let command = crate::commands::WorldBorder::new(crate::types::world::WorldBorderAction::Center {
            x: 100.5,
            z: -100.5,
        });
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn set_damage() {
        let command = crate::commands::WorldBorder::new(crate::types::world::WorldBorderAction::DamageAmount {
            damage: 5.0,
        });
        assert_validation!(command.to_string(), true);
    }

    #[tokio::test]
    async fn set_warning() {
        let command = crate::commands::WorldBorder::new(crate::types::world::WorldBorderAction::Warning {
            warning_type: crate::types::world::WorldBorderWarning::Distance,
            distance: 10,
        });
        assert_validation!(command.to_string(), true);
    }