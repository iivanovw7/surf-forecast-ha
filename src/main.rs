use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tera::Tera;
use tokio::fs;

const OPTIONS_PATH: &str = "/data/options.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct NotificationSettings {
    pub enable: bool,
    pub min_wave_height: f32,
    pub max_wind_kts: u32,
    pub webhook_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddonOptions {
    pub spots: Vec<String>,
    pub notifications: NotificationSettings,
}

impl Default for AddonOptions {
    fn default() -> Self {
        Self {
            spots: vec![
                "Ribeira d'Ilhas".to_string(),
                "Supertubos".to_string(),
                "Guincho".to_string(),
            ],
            notifications: NotificationSettings {
                enable: true,
                min_wave_height: 1.5,
                max_wind_kts: 15,
                webhook_url: Some("http://supervisor/core/api/events/surf_alert".to_string()),
            },
        }
    }
}

#[derive(Clone)]
struct AppState {
    tera: Arc<Tera>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tera = Tera::default();

    tera.load_from_glob("templates/**/*.tera")?;

    let state = AppState {
        tera: Arc::new(tera),
    };

    let app = Router::new()
        .route("/", get(render_index))
        .route("/api/config", get(get_config_api))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Surf Backend with Tera running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}

async fn render_index(State(state): State<AppState>) -> impl IntoResponse {
    let options = load_options().await;
    let mut context = tera::Context::new();

    context.insert("spots", &options.spots);
    context.insert("notifications", &options.notifications);
    context.insert("page_title", "Surf Analytics Dashboard");

    match state.tera.render("index.tera", &context) {
        Ok(rendered_html) => Html(rendered_html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template rendering failed: {}", err),
        )
            .into_response(),
    }
}

async fn get_config_api() -> Json<AddonOptions> {
    Json(load_options().await)
}

async fn load_options() -> AddonOptions {
    if Path::new(OPTIONS_PATH).exists() {
        if let Ok(data) = fs::read_to_string(OPTIONS_PATH).await {
            if let Ok(options) = serde_json::from_str::<AddonOptions>(&data) {
                return options;
            }
        }
    }
    AddonOptions::default()
}
