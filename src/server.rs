/* src/server.rs */

use crate::config::Config;
use crate::handler::main_handler;
use axum::Router;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub public_dir: PathBuf,
    pub index_router_mode: bool,
}

pub fn create_router(config: Config) -> Router {
    let app_state = Arc::new(AppState {
        public_dir: config.public_dir.clone(),
        index_router_mode: config.index_router_mode,
    });

    // **FIX**: Use `.fallback()` for handlers instead of `.fallback_service()`.
    // `.fallback()` is the correct and idiomatic way to register a catch-all handler.
    Router::new().fallback(main_handler).with_state(app_state)
}
