use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema;

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
    // TODO: Can this be derived?
    pub fn from_str(value: impl AsRef<str>) -> Option<Self> {
        match value.as_ref() {
            "Sunny" => Some(Self::Sunny),
            "Cloudy" => Some(Self::Cloudy),
            "Rain" => Some(Self::Rain),
            "Snow" => Some(Self::Snow),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sunny => "Sunny",
            Self::Cloudy => "Cloudy",
            Self::Rain => "Rain",
            Self::Snow => "Snow",
        }
    }
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = schema::weather_forecast)]
pub struct WeatherForecast {
    pub region: String,

    pub forecast: String,
    pub valid_until: chrono::NaiveDateTime,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
