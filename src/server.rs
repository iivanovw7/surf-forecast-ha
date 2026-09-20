use crate::handlers;
use crate::types::config::AppState;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tera::Tera;
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

    tera.load_from_glob("templates/**/*.tera")?;

    tracing::info!("initializing state");

    let state = AppState {
        tera: Arc::new(tera),
    };

    tracing::info!("initializing router");

    let router = Router::new()
        .route("/", get(handlers::index::get))
        .route("/api/config", get(handlers::config::get))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::debug!("Router initialized, now listening on port {}", 8080);

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
