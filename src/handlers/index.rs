use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};

use crate::{handlers::config::load_options, types::config::AppState};

pub async fn get(State(state): State<AppState>) -> impl IntoResponse {
    let options = load_options().await;
    let mut context = tera::Context::new();

    context.insert("spots", &options.spots);
    context.insert("page_title", "Surf Analytics Dashboard");

    let mut headers = HeaderMap::new();

    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store, no-cache, must-revalidate, max-age=0"),
    );

    headers.insert(header::PRAGMA, header::HeaderValue::from_static("no-cache"));

    match state.tera.render("index.tera", &context) {
        Ok(rendered_html) => (headers, Html(rendered_html)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template rendering failed: {}", err),
        )
            .into_response(),
    }
}
