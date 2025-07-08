use axum::{Json, extract::Path};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::dtos::*;
use super::models::*;
use crate::{error::ErrorResponse, get_db, schema};

pub fn get_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(weather_forecast, upsert_forecast, delete_region))
        .routes(routes!(list_regions))
}

#[utoipa::path(
    get,
    path = "/weather/{region}",
    params(
        ("region" = String, Path, description = "Region for which to get the weather forecast."),
    ),
    tag = "WeatherService",
    responses(
        (status = 200, description = "Weather forecast available.", body = WeatherForecastResponse),
        (status = 404, description = "Region not found.", body = ErrorResponse)
    )
)]
/// Get current weather forecast for a region, 80% chance of being available.
async fn weather_forecast(
    Path(region): Path<String>,
) -> Result<Json<WeatherForecastResponse>, ErrorResponse> {
    let mut conn = get_db!();

    let forecast = schema::weather_forecast::table
        .filter(schema::weather_forecast::region.eq(region))
        .first::<WeatherForecast>(&mut conn)
        .await
        .map_err(|_| ErrorResponse::not_found("Region not found."))?;

    Ok(Json(WeatherForecastResponse {
        forecast: Weather::from_str(&forecast.forecast).ok_or(
            ErrorResponse::internal_server_error(
                "Invalid weather forecast format in the database...",
            ),
        )?,
        valid_until: forecast.valid_until,
    }))
}

#[utoipa::path(
    put,
    path = "/weather/{region}",
    params(
        ("region" = String, Path, description = "Region name to insert or update."),
    ),
    tag = "WeatherService",
    responses(
        (status = 200, description = "Region updated successfully.", body = WeatherForecastResponse),
        (status = 201, description = "Region created successfully.", body = WeatherForecastResponse)
    )
)]
/// Insert or update a weather forecast for a region.
async fn upsert_forecast(
    Path(region): Path<String>,
    Json(payload): Json<RegionUpsertRequest>,
) -> Result<Json<WeatherForecastResponse>, ErrorResponse> {
    let mut conn = get_db!();

    let now = chrono::Utc::now().naive_utc();
    let valid_until = now + chrono::Duration::hours(payload.valid_for_hours as i64);
    let new_forecast = WeatherForecast {
        region: region.clone(),
        forecast: payload.forecast.as_str().to_string(),
        valid_until,
        created_at: now,
        updated_at: now,
    };

    let result = diesel::insert_into(schema::weather_forecast::table)
        .values(&new_forecast)
        .on_conflict(schema::weather_forecast::region)
        .do_update()
        .set((
            schema::weather_forecast::forecast.eq(&new_forecast.forecast),
            schema::weather_forecast::valid_until.eq(&new_forecast.valid_until),
            schema::weather_forecast::updated_at.eq(now),
        ))
        .get_result::<WeatherForecast>(&mut conn)
        .await
        .map_err(|_| ErrorResponse::internal_server_error("Failed to upsert region."))?;

    Ok(Json(WeatherForecastResponse {
        forecast: Weather::from_str(&result.forecast).ok_or(
            ErrorResponse::internal_server_error(
                "Invalid weather forecast format in the database...",
            ),
        )?,
        valid_until: result.valid_until,
    }))
}

#[utoipa::path(
    delete,
    path = "/weather/{region}",
    params(
        ("region" = String, Path, description = "Region name to delete."),
    ),
    tag = "WeatherService",
    responses(
        (status = 204, description = "Region deleted successfully."),
        (status = 404, description = "Region not found.", body = ErrorResponse)
    )
)]
/// Delete a weather forecast for a region.
async fn delete_region(Path(region): Path<String>) -> Result<(), ErrorResponse> {
    let mut conn = get_db!();

    let deleted = diesel::delete(schema::weather_forecast::table)
        .filter(schema::weather_forecast::region.eq(region))
        .execute(&mut conn)
        .await
        .map_err(|_| ErrorResponse::internal_server_error("Failed to delete region."))?;

    if deleted == 0 {
        return Err(ErrorResponse::not_found("Region not found."));
    }

    Ok(())
}

#[utoipa::path(
    get,
    path = "/regions",
    tag = "WeatherService",
    responses(
        (status = 200, description = "List of all regions.", body = RegionListResponse)
    )
)]
/// List all regions with weather forecasts.
async fn list_regions() -> Result<Json<RegionListResponse>, ErrorResponse> {
    let mut conn = get_db!();

    let regions = schema::weather_forecast::table
        .select(schema::weather_forecast::region)
        .distinct()
        .load::<String>(&mut conn)
        .await
        .map_err(|_| ErrorResponse::internal_server_error("Failed to fetch regions."))?;

    Ok(Json(RegionListResponse { regions }))
}
