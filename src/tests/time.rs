use crate::assert_validation;

#[tokio::test]
async fn add_time() {
    let command = crate::commands::Time::new(crate::types::world::TimeAction::Add(100));

    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn set_time() {
    let command = crate::commands::Time::new(crate::types::world::TimeAction::Set(
        crate::types::TimeValue::Night,
    ));

    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn query_time() {
    let command = crate::commands::Time::new(crate::types::world::TimeAction::Query(
        crate::types::world::TimeQueryType::DayTime,
    ));

    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn query_time_format() {
    let command = crate::commands::Time::new(crate::types::world::TimeAction::Query(
        crate::types::world::TimeQueryType::GameTime,
    ));

    assert_validation!(command.to_string(), true);
}


