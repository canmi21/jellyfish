/* src/server.rs */

use crate::config::Config;
use crate::handler::fallback_handler;
use axum::handler::Handler;
use axum::{
    Router,
    body::Body,
    extract::State,
    http::Request,
    response::IntoResponse,
    routing::{get, get_service},
};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;

// Define a struct to hold all shared state.
#[derive(Clone)]
pub struct AppState {
    pub public_dir: PathBuf,
    pub index_router_mode: bool,
}

// This is a new, specific handler for the root path ("/").
// Its purpose is to intercept the request for "/" before `ServeDir` can process it.
// This ensures the root path always respects the logic in `fallback_handler`
// (like checking for SPA mode) instead of just serving `index.html` by default.
async fn root_handler(State(state): State<Arc<AppState>>, req: Request<Body>) -> impl IntoResponse {
    // This handler simply calls the main fallback_handler.
    // This ensures consistent logic for both the root path and any other "not found" paths.
    fallback_handler(State(state), req).await
}

pub fn create_router(config: Config) -> Router {
    // Create an instance of our shared state.
    let app_state = Arc::new(AppState {
        public_dir: config.public_dir.clone(),
        index_router_mode: config.index_router_mode,
    });

    // --- ROUTING LOGIC CHANGE ---

    // 1. Create the static file service (ServeDir).
    // We attach our main fallback_handler as the fallback for ServeDir itself.
    // This means: if ServeDir cannot find a requested file (e.g., "/non-existent.css"),
    // it will then call our custom fallback_handler.
    let serve_dir_with_fallback =
        ServeDir::new(config.public_dir).fallback(fallback_handler.with_state(app_state.clone()));

    // 2. Build the main application router with the corrected logic.
    Router::new()
        // 2A. Define an explicit route for "GET /".
        // Routes are checked *before* fallback services. By defining this route,
        // we guarantee that requests for the root path are handled by our `root_handler`,
        // which then uses our custom SPA/404 logic.
        .route("/", get(root_handler))
        // 2B. For any request that does *not* match a route above (e.g., "/style.css" or "/some/route"),
        // pass it to our `serve_dir_with_fallback` service.
        .fallback_service(
            get_service(serve_dir_with_fallback).handle_error(|error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Static file service error: {}", error),
                )
            }),
        )
        // Provide the AppState to the router so our handlers can access it.
        .with_state(app_state)
}
