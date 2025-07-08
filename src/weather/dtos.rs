use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, schema};

use super::models::*;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct WeatherForecastResponse {
    /// Current weather
    pub forecast: Weather,
    /// When is the forecast not valid anymore
    pub valid_until: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RegionUpsertRequest {
    /// Weather forecast for the region
    pub forecast: Weather,
    /// For how many hours the forecast considered is valid
    #[schema(example = 4)]
    pub valid_for_hours: u64,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RegionListResponse {
    pub regions: Vec<String>,
}
