/* src/handler.rs */

use crate::server::AppState; // Import the shared state struct
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::{Html, IntoResponse},
};
use std::sync::Arc;

/// The intelligent fallback handler.
pub async fn fallback_handler(
    State(state): State<Arc<AppState>>,
    _req: Request<Body>,
) -> impl IntoResponse {
    // Use the shared state to get the public directory path
    let public_dir = &state.public_dir;
    let index_path = public_dir.join("index.html");
    let custom_404_path = public_dir.join("404.html");

    // **CORE LOGIC CHANGE**: Only attempt SPA fallback if the mode is true.
    if state.index_router_mode {
        if let Ok(content) = tokio::fs::read_to_string(index_path).await {
            return (StatusCode::OK, Html(content)).into_response();
        }
    }

    // Both modes use the custom 404 page if it exists.
    if let Ok(content) = tokio::fs::read_to_string(custom_404_path).await {
        return (StatusCode::NOT_FOUND, Html(content)).into_response();
    }

    // Finally, fall back to the built-in static 404 page.
    let static_404_content = include_str!("../index/404.html");
    (StatusCode::NOT_FOUND, Html(static_404_content)).into_response()
}
