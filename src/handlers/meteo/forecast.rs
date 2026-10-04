use std::time::{Duration, Instant};

use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};

use chrono::Utc;
use tokio::time::sleep;

use crate::{
    handlers::{
        config::load_options,
        meteo::transformers::{build_daily_forecast, build_day_headers},
    },
    types::{
        config::{AppState, Spot},
        forecast::{MarineForecast, SpotView},
    },
    utils::context::create_context,
};

const SPOT_REQUEST_DELAY: u64 = 5;

pub async fn overview(State(state): State<AppState>) -> impl IntoResponse {
    let options = load_options().await;

    let days_header = build_day_headers(Utc::now().date_naive());

    let spots: Vec<SpotView> = options
        .spots
        .iter()
        .map(|spot| SpotView {
            id: spot.id.clone(),
            name: spot.name.clone(),
            location: spot.region.clone().unwrap_or_default(),
            cover: spot.normalized_cover().unwrap_or_default(),
            has_data: false,
            days: Vec::new(),
        })
        .collect();

    let mut context = create_context(&State(state.clone()));

    let date_range_label =
        if let (Some(first), Some(last)) = (days_header.first(), days_header.last()) {
            format!("{} - {}", first.date_str, last.date_str)
        } else {
            "No forecast data".to_string()
        };

    context.insert("page_title", "Surf Forecast");
    context.insert("spots", &spots);
    context.insert("days_header", &days_header);
    context.insert("model", &options.model);
    context.insert("total_spots_count", &spots.len());
    context.insert("date_range_label", &date_range_label);
    context.insert("best_conditions_count", &0usize);

    let rendered = match state.tera.render("pages/overview/overview.html", &context) {
        Ok(html) => html,
        Err(error) => {
            tracing::error!(
                error = %error,
                "failed to render overview page"
            );

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render overview page",
            )
                .into_response();
        }
    };

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CACHE_CONTROL,
        "no-store, no-cache, must-revalidate".parse().unwrap(),
    );

    (headers, Html(rendered)).into_response()
}

pub async fn spot(State(state): State<AppState>, Path(spot_id): Path<String>) -> impl IntoResponse {
    let options = load_options().await;

    let spot = match options.spots.iter().find(|spot| spot.id == spot_id) {
        Some(spot) => spot,
        None => {
            tracing::warn!(
                spot_id = %spot_id,
                "requested unknown surf spot"
            );

            return (
                StatusCode::NOT_FOUND,
                format!("Unknown surf spot: {spot_id}"),
            )
                .into_response();
        }
    };

    let selected_model = options.model;

    let forecast = match fetch_spot_forecast(&state, spot).await {
        Ok(forecast) => forecast,
        Err(error) => {
            tracing::error!(
                spot_id = %spot.id,
                spot_name = %spot.name,
                error = %error,
                "failed to fetch surf forecast"
            );

            return (
                StatusCode::BAD_GATEWAY,
                format!("Failed to fetch forecast for {}", spot.name),
            )
                .into_response();
        }
    };

    let days = build_daily_forecast(&forecast, selected_model);

    let mut spot = spot.clone();
    spot.cover = spot.normalized_cover();

    let mut context = tera::Context::new();

    context.insert("spot", &spot);
    context.insert("days", &days);

    let rendered = match state
        .tera
        .render("pages/overview/ui/grid-row.html", &context)
    {
        Ok(html) => html,
        Err(error) => {
            tracing::error!(
                spot_id = %spot.id,
                spot_name = %spot.name,
                error = %error,
                "failed to render spot forecast"
            );

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render spot forecast",
            )
                .into_response();
        }
    };

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CACHE_CONTROL,
        "no-store, no-cache, must-revalidate".parse().unwrap(),
    );

    headers.insert(
        header::CONTENT_TYPE,
        "text/html; charset=utf-8".parse().unwrap(),
    );

    (headers, Html(rendered)).into_response()
}

async fn fetch_spot_forecast(state: &AppState, spot: &Spot) -> Result<MarineForecast, String> {
    let mut last_completed = state.forecast_queue.lock().await;

    if let Some(previous_completion) = *last_completed {
        let cooldown = Duration::from_secs(SPOT_REQUEST_DELAY);
        let elapsed = previous_completion.elapsed();

        if elapsed < cooldown {
            let remaining = cooldown - elapsed;

            tracing::debug!(
                spot_id = %spot.id,
                wait_seconds = remaining.as_secs_f64(),
                "waiting for Open-Meteo cooldown"
            );

            sleep(remaining).await;
        }
    }

    tracing::info!(
        spot_id = %spot.id,
        spot_name = %spot.name,
        latitude = spot.latitude,
        longitude = spot.longitude,
        "requesting Open-Meteo forecast"
    );

    let result =
        crate::handlers::meteo::open_meteo::get_meteo_data(spot.latitude, spot.longitude).await;

    *last_completed = Some(Instant::now());

    drop(last_completed);

    result.map_err(|error| format!("{error:?}"))
}
