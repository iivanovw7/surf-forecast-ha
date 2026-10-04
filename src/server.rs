use crate::handlers;
use crate::types::config::AppState;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tera::Tera;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub async fn server() -> anyhow::Result<()> {
    let mut tera = Tera::default();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("surf_forecast_ha=debug")),
        )
        .with(fmt::layer())
        .init();

    let app_root = crate::utils::context::get_app_root();
    let templates_pattern = app_root.join("templates/**/*.html").to_string_lossy().to_string();
    tera.load_from_glob(&templates_pattern)?;

    tracing::info!("initializing state");

    let state = AppState {
        tera: Arc::new(tera),
        forecast_queue: Arc::new(Mutex::new(None)),
    };

    tracing::info!("initializing router");

    let assets_path = app_root.join("assets");
    let assets_serve = ServeDir::new(assets_path);

    let router = Router::new()
        .route("/", get(handlers::meteo::forecast::overview))
        .route("/api/config", get(handlers::config::get))
        .route(
            "/api/forecast/{spot_id}",
            get(handlers::meteo::forecast::spot),
        )
        .nest_service("/assets", assets_serve)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::debug!("router initialized, now listening on port {}", 8080);

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
