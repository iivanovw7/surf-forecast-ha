use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::{handlers::config::load_options, types::config::AppState};

pub async fn get(State(state): State<AppState>) -> impl IntoResponse {
    let options = load_options().await;
    let mut context = tera::Context::new();

    context.insert("spots", &options.spots);
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
