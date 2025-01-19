use crate::assert_validation;

#[tokio::test]
async fn add_experience_points() {
    let command = crate::commands::XP::new(crate::types::experience::EXPAction::Add {
        selector: crate::types::selector::TargetSelector::Current(None),
        etype: crate::types::experience::EXP::Points(100),
    });
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn add_experience_levels() {
    let command = crate::commands::XP::new(crate::types::experience::EXPAction::Add {
        selector: crate::types::selector::TargetSelector::Current(None),
        etype: crate::types::experience::EXP::Levels(5),
    });
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn set_experience() {
    let command = crate::commands::XP::new(crate::types::experience::EXPAction::Set {
        selector: crate::types::selector::TargetSelector::Current(None),
        etype: crate::types::experience::EXP::Points(1000),
    });
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn query_experience() {
    let command = crate::commands::XP::new(crate::types::experience::EXPAction::Query {
        selector: crate::types::selector::TargetSelector::Current(None),
        etype: crate::types::experience::EXP::Points(0),
    });
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn with_different_selectors() {
    let command = crate::commands::XP::new(crate::types::experience::EXPAction::Add {
        selector: crate::types::selector::TargetSelector::Nearest(None),
        etype: crate::types::experience::EXP::Levels(1),
    });
    assert_validation!(command.to_string(), true);
}
