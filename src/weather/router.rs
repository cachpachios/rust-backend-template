use axum::Json;
use rand::Rng;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::models::*;
use crate::error::ErrorResponse;

pub fn get_router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(weather_forecast))
}

#[utoipa::path(
    get,
    path = "/forecast",
    tag = "WeatherService",
    responses(
        (status = 200, description = "Weather forecast available.", body = WeatherForecast),
        (status = 503, description = "Weather service is currently unavailable", body = ErrorResponse),
    )
)]
/// Get current weather forecast, 80% chance of being available.
async fn weather_forecast() -> Result<Json<WeatherForecast>, ErrorResponse> {
    if rand::rng().random::<f64>() > 0.8 {
        return Err(ErrorResponse::service_unavailable(
            "Weather service is currently unavailable. Try again.",
        ));
    }

    Ok(Json(WeatherForecast {
        current: Weather::random(),
        forecast: Weather::random(),
        time_until_forecast: rand::rng().random_range(0..7200), // Random time between 0 and 2 hours
    }))
}
