use crate::assert_validation;

#[tokio::test]
async fn clear_weather() {
    let command =
        crate::commands::Weather::new(crate::types::world::WeatherType::Clear("6000".to_string()));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn clear_weather_no_duration() {
    let command =
        crate::commands::Weather::new(crate::types::world::WeatherType::Clear("".to_string()));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn rain_weather() {
    let command =
        crate::commands::Weather::new(crate::types::world::WeatherType::Rain("12000".to_string()));
    assert_validation!(command.to_string(), true);
}

#[tokio::test]
async fn thunder_weather() {
    let command = crate::commands::Weather::new(crate::types::world::WeatherType::Thunder(
        "24000".to_string(),
    ));
    assert_validation!(command.to_string(), true);
}
