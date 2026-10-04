use axum::Json;
use std::path::Path;
use tokio::fs;

use crate::types::config::{AddonOptions, OPTIONS_PATH};
#[cfg(debug_assertions)]
use crate::types::config::{Config, CONFIG_PATH};

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
        let app_root = crate::utils::context::get_app_root();
        let config_path_buf = app_root.join(CONFIG_PATH);
        let path_to_check = if Path::new(CONFIG_PATH).exists() {
            std::path::PathBuf::from(CONFIG_PATH)
        } else {
            config_path_buf
        };

        if path_to_check.exists() {
            match fs::read_to_string(&path_to_check).await {
                Ok(data) => match serde_yaml::from_str::<Config>(&data) {
                    Ok(config_yaml) => return config_yaml.options,
                    Err(err) => {
                        tracing::error!("Failed to parse yaml at {:?}: {}", path_to_check, err);
                    }
                },
                Err(err) => {
                    tracing::error!("Failed to read config file at {:?}: {}", path_to_check, err);
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
