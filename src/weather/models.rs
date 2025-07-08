use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema, PartialEq, Clone, Copy)]
/// Known weather conditions
pub enum Weather {
    /// Sunny weather
    Sunny,
    /// Cloudy weather
    Cloudy,
    /// Rainy weather
    Rain,
    /// Snowy weather
    Snow,
}

impl Weather {
    pub fn random() -> Self {
        let weathers = [
            Weather::Sunny,
            Weather::Cloudy,
            Weather::Rain,
            Weather::Snow,
        ];
        let mut rng = rand::rng();
        let idx = rand::Rng::random_range(&mut rng, 0..weathers.len());
        weathers[idx]
    }
}

#[derive(Serialize, Deserialize, Debug, ToSchema, PartialEq, Clone)]
pub struct WeatherForecast {
    /// Current weather
    #[schema(example = "Sunny")]
    pub current: Weather,
    /// Forecast weather for the next period
    #[schema(example = "Cloudy")]
    pub forecast: Weather,
    /// Time until the next period is expected to start in seconds (forecast will become current)
    #[schema(example = 3600)]
    pub time_until_forecast: u64,
}
