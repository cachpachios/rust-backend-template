use std::env;

use utoipa::openapi::security::{Http, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use backend_template::weather::get_router as get_weather_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Define the OpenAPI documentation
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Rust Backend Template",
            version = "1.0.0",
            description = "A template for a Rust backend project using Axum.",
        ),
        tags(
            (name = "WeatherService", description = "Example weather service to give forecasts!."),
        )
    )]
    struct ApiDoc;

    let (router, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(get_weather_router())
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on {addr}");

    axum::serve(listener, router).await.unwrap();
}
