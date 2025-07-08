-- Your SQL goes here

CREATE TABLE "weather_forecast" (
    "region" TEXT NOT NULL PRIMARY KEY,
    "forecast" TEXT NOT NULL,
    "valid_until" TIMESTAMP NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
