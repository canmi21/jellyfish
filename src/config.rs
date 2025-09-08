/* src/config.rs */

use anyhow::Result;
use fancy_log::{LogLevel, log, set_log_level};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub addr: SocketAddr,
    pub public_dir: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        // Parse LOG_LEVEL and configure fancy-log
        let level = std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .to_lowercase();
        let log_level = match level.as_str() {
            "debug" => LogLevel::Debug,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        };
        set_log_level(log_level);

        let port: u16 = std::env::var("BIND_PORT")
            .unwrap_or_else(|_| "33433".to_string())
            .parse()?;
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        let public_dir_str =
            std::env::var("PUBLIC_DIR").unwrap_or_else(|_| "~/jellyfish/public".to_string());

        let public_dir = PathBuf::from(shellexpand::tilde(&public_dir_str).into_owned());

        Ok(Self { addr, public_dir })
    }
}

pub fn setup_public_dir(public_dir: &PathBuf) -> Result<()> {
    if !public_dir.exists() {
        log(
            LogLevel::Info,
            &format!(
                "Public directory not found, creating '{}'",
                public_dir.display()
            ),
        );
        std::fs::create_dir_all(public_dir)?;

        let default_index_content = include_str!("../index/index.html");
        let index_path = public_dir.join("index.html");
        std::fs::write(index_path, default_index_content)?;
        log(LogLevel::Info, "Created a default index.html");
    }
    Ok(())
}
