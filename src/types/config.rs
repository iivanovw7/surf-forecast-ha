use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::Tera;

pub const OPTIONS_PATH: &str = "/data/options.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct AddonOptions {
    pub spots: Vec<String>,
}

impl Default for AddonOptions {
    fn default() -> Self {
        Self { spots: vec![] }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<Tera>,
}
