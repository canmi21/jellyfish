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
    log(
        LogLevel::Info,
        &format!(
            "Index Router Mode (SPA Fallback): {}",
            config.index_router_mode
        ),
    );

    setup_public_dir(&config.public_dir)?;
    let addr = config.addr;
    let app = server::create_router(config);
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::signal_handler())
        .await?;

    Ok(())
}
