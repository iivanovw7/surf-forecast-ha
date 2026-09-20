use axum::Json;
use std::path::Path;
use tokio::fs;

use crate::types::config::{AddonOptions, OPTIONS_PATH};

pub async fn load_options() -> AddonOptions {
    if Path::new(OPTIONS_PATH).exists() {
        if let Ok(data) = fs::read_to_string(OPTIONS_PATH).await {
            if let Ok(options) = serde_json::from_str::<AddonOptions>(&data) {
                return options;
            }
        }
    }

    AddonOptions::default()
}

pub async fn get() -> Json<AddonOptions> {
    Json(load_options().await)
}
