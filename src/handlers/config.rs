use axum::Json;
use std::path::Path;
use tokio::fs;

use crate::types::config::{AddonOptions, Config, CONFIG_PATH, OPTIONS_PATH};

pub async fn load_options() -> AddonOptions {
    if Path::new(OPTIONS_PATH).exists() {
        match fs::read_to_string(OPTIONS_PATH).await {
            Ok(data) => match serde_json::from_str::<AddonOptions>(&data) {
                Ok(options) => return options,
                Err(err) => {
                    tracing::error!("Failed to parse options at {}: {}", OPTIONS_PATH, err);
                }
            },
            Err(err) => {
                tracing::error!("Failed to read options file at {}: {}", OPTIONS_PATH, err);
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        if Path::new(CONFIG_PATH).exists() {
            match fs::read_to_string(CONFIG_PATH).await {
                Ok(data) => match serde_yaml::from_str::<Config>(&data) {
                    Ok(config_yaml) => return config_yaml.options,
                    Err(err) => {
                        tracing::error!("Failed to parse yaml at {}: {}", CONFIG_PATH, err);
                    }
                },
                Err(err) => {
                    tracing::error!("Failed to read config file at {}: {}", CONFIG_PATH, err);
                }
            }
        }
    }

    AddonOptions::default()
}

pub async fn get() -> impl axum::response::IntoResponse {
    let mut headers = axum::http::HeaderMap::new();

    headers.insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::header::HeaderValue::from_static(
            "no-store, no-cache, must-revalidate, max-age=0",
        ),
    );
    headers.insert(
        axum::http::header::PRAGMA,
        axum::http::header::HeaderValue::from_static("no-cache"),
    );

    (headers, Json(load_options().await))
}
