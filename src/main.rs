/* src/main.rs */

mod config;
mod handler;
mod server;
mod shutdown;

use crate::config::{Config, setup_public_dir};
use fancy_log::{LogLevel, log};
use lazy_motd::lazy_motd;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // --- Initialization ---
    let config = Config::from_env()?;

    lazy_motd!(bin = "jellyfish");
    log(LogLevel::Info, "Jellyfish server starting...");
    log(
        LogLevel::Info,
        &format!("Serving files from: {}", config.public_dir.display()),
    );
    log(
        LogLevel::Info,
        &format!("Listening on: http://{}", config.addr),
    );

    // Check and create public directory if it doesn't exist
    setup_public_dir(&config.public_dir)?;

    // --- Create and run server ---
    let app = server::create_router(config.public_dir);
    let listener = TcpListener::bind(&config.addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::signal_handler())
        .await?;

    Ok(())
}
