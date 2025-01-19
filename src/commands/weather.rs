use crate::types::world::WeatherType;

use super::MinecraftCommand;



#[derive(Debug, Clone)]
pub struct Weather(WeatherType);

impl Weather {
    pub fn new(weather_type: WeatherType) -> Self {
        Weather(weather_type)
    }
}

impl ToString for Weather {
    fn to_string(&self) -> String {
        format!("weather {}", self.0.to_string())
    }
}

impl MinecraftCommand for Weather {}
