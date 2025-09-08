/* src/server.rs */

use crate::handler::fallback_handler;
use axum::{Router, routing::get_service};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub fn create_router(public_dir: PathBuf) -> Router {
    let shared_public_dir = Arc::new(public_dir.clone());

    // 1. Create a dedicated router FOR the fallback logic.
    // This router's only job is to host our handler. Because a Router
    // implements the `Service` trait, it can be used by `ServeDir`.
    let fallback_service = Router::new()
        .fallback(fallback_handler)
        // Provide the necessary state to the handler within this fallback router.
        .with_state(shared_public_dir);

    // 2. Configure the main static file service.
    // When ServeDir can't find a file, it will forward the request
    // to the `fallback_service` we just created.
    let serve_dir_service = ServeDir::new(public_dir).fallback(fallback_service);

    // 3. The main application router delegates ALL requests to our composite service.
    Router::new().fallback_service(get_service(serve_dir_service).handle_error(
        |error| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Static file service error: {}", error),
            )
        },
    ))
}
