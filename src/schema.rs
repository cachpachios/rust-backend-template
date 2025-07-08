// @generated automatically by Diesel CLI.

diesel::table! {
    weather_forecast (region) {
        region -> Text,
        forecast -> Text,
        valid_until -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
