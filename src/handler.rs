/* src/handler.rs */

use axum::{
    body::Body,
    // The State extractor is needed to access shared state
    extract::State,
    http::{Request, StatusCode},
    // Removed unused 'Response' import
    response::{Html, IntoResponse},
};
use std::path::PathBuf;
use std::sync::Arc;

/// The fallback handler for SPA routing and custom 404 pages.
/// It receives the public directory path via the State extractor.
pub async fn fallback_handler(
    State(public_dir): State<Arc<PathBuf>>,
    _req: Request<Body>,
) -> impl IntoResponse {
    let index_path = public_dir.join("index.html");
    let custom_404_path = public_dir.join("404.html");

    // 1. SPA Fallback
    if let Ok(content) = tokio::fs::read_to_string(index_path).await {
        return (StatusCode::OK, Html(content)).into_response();
    }

    // 2. User-provided custom 404 page
    if let Ok(content) = tokio::fs::read_to_string(custom_404_path).await {
        return (StatusCode::NOT_FOUND, Html(content)).into_response();
    }

    // 3. Built-in 404 page
    // Corrected path for include_str!
    let static_404_content = include_str!("../index/404.html");
    (StatusCode::NOT_FOUND, Html(static_404_content)).into_response()
}
