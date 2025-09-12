/* src/handler.rs */

use crate::response::{error, success};
use crate::server::AppState;
use axum::{
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, HeaderValue, Request, StatusCode, header},
    response::{Html, IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use fancy_log::{LogLevel, log};
use mime_guess;
use path_clean::PathClean;
use serde::{Deserialize, Deserializer};
use serde_json::json;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use xxhash_rust::xxh64::xxh64;

fn deserialize_flag<'de, D>(deserializer: D) -> Result<Option<()>, D::Error>
where
    D: Deserializer<'de>,
{
    let _ = String::deserialize(deserializer)?;
    Ok(Some(()))
}

#[derive(Deserialize, Debug)]
pub struct ApiParams {
    #[serde(default, deserialize_with = "deserialize_flag")]
    info: Option<()>,
    #[serde(default, deserialize_with = "deserialize_flag")]
    list: Option<()>,
}

/// The single, powerful handler for all incoming GET requests.
pub async fn main_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ApiParams>,
    req: Request<Body>,
) -> Response {
    // --- 1. Decode and Sanitize Path ---
    let requested_path_str = req.uri().path().to_string(); // Keep a copy for logging
    log(
        LogLevel::Debug,
        &format!("-> New request for URI: {}", requested_path_str),
    );

    let decoded_path = percent_encoding::percent_decode_str(&requested_path_str)
        .decode_utf8_lossy()
        .to_string();
    let path = PathBuf::from(decoded_path);

    let clean_path = path.clean();
    for component in clean_path.components() {
        if let Component::ParentDir = component {
            log(
                LogLevel::Debug,
                "   Path traversal detected. Responding with 403 FORBIDDEN.",
            );
            return error(StatusCode::FORBIDDEN, "Path traversal is not allowed.");
        }
    }
    let safe_relative_path = clean_path.strip_prefix("/").unwrap_or(&clean_path);
    let resource_path = state.public_dir.join(safe_relative_path);
    log(
        LogLevel::Debug,
        &format!("   Sanitized resource path: {}", resource_path.display()),
    );

    // --- 2. Handle API calls ONLY when NOT in SPA mode ---
    if !state.index_router_mode {
        if params.info.is_some() {
            log(LogLevel::Debug, "   API call detected: ?info. Handling...");
            return api_get_file_info(
                &resource_path,
                safe_relative_path.to_string_lossy().as_ref(),
            )
            .await;
        }
        if params.list.is_some() {
            log(LogLevel::Debug, "   API call detected: ?list. Handling...");
            return api_list_directory(&resource_path).await;
        }
    }

    // --- 3. Attempt to serve a static file ---
    log(
        LogLevel::Debug,
        "   Attempting to serve static file via ServeDir...",
    );
    match ServeDir::new(&state.public_dir).oneshot(req).await {
        Ok(res) => {
            log(
                LogLevel::Debug,
                &format!("   ServeDir response status: {}", res.status()),
            );

            if res.status() == StatusCode::NOT_FOUND {
                // --- FIX: WORKAROUND for files with unknown MIME types ---
                log(
                    LogLevel::Debug,
                    "   ServeDir returned 404. Manually checking if file exists...",
                );
                if resource_path.is_file() {
                    log(
                        LogLevel::Debug,
                        &format!(
                            "   Manual check FOUND file at {}. Serving with guessed MIME type.",
                            resource_path.display()
                        ),
                    );
                    if let Ok(content) = tokio::fs::read(&resource_path).await {
                        let mime_type =
                            mime_guess::from_path(&resource_path).first_or_octet_stream();
                        let mut headers = HeaderMap::new();
                        headers.insert(
                            header::CONTENT_TYPE,
                            HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                        );
                        return (headers, content).into_response();
                    }
                } else {
                    log(LogLevel::Debug, "   Manual check did NOT find file.");
                }

                // --- Fallback Logic ---
                if state.index_router_mode {
                    log(LogLevel::Debug, "   Entering SPA fallback logic...");
                    let index_path = state.public_dir.join("index.html");
                    if let Ok(content) = tokio::fs::read_to_string(&index_path).await {
                        return (StatusCode::OK, Html(content)).into_response();
                    } else {
                        return error(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "SPA mode is on, but index.html could not be found.",
                        );
                    }
                } else {
                    log(
                        LogLevel::Debug,
                        "   Entering standard 404 fallback logic...",
                    );
                    let custom_404_path = state.public_dir.join("404.html");
                    if let Ok(content) = tokio::fs::read_to_string(custom_404_path).await {
                        return (StatusCode::NOT_FOUND, Html(content)).into_response();
                    }
                    let static_404_content = include_str!("../index/404.html");
                    return (StatusCode::NOT_FOUND, Html(static_404_content)).into_response();
                }
            }
            return res.into_response();
        }
        Err(e) => {
            log(
                LogLevel::Error,
                &format!("   Static file service internal error: {}", e),
            );
            return error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Static file service error: {}", e),
            );
        }
    };
}

// (The two API helper functions below are unchanged)
// ... (rest of the file is the same)
/// API logic to get information about a single file.
async fn api_get_file_info(path: &Path, relative_path: &str) -> Response {
    match tokio::fs::metadata(path).await {
        Ok(meta) => {
            if !meta.is_file() {
                return error(
                    StatusCode::BAD_REQUEST,
                    "The '?info' parameter can only be used on files.",
                );
            }
            let modified_time: DateTime<Utc> = meta
                .modified()
                .unwrap_or(std::time::SystemTime::now())
                .into();
            match tokio::fs::read(path).await {
                Ok(content) => {
                    let hash = xxh64(&content, 0);
                    let info = json!({
                        "path": relative_path,
                        "size_bytes": meta.len(),
                        "modified_time": modified_time.to_rfc3339(),
                        "hash_xxh64": format!("{:x}", hash),
                    });
                    success(Some(info))
                }
                Err(e) => error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read file: {}", e),
                ),
            }
        }
        Err(_) => error(StatusCode::NOT_FOUND, "Resource not found."),
    }
}

/// API logic to list the contents of a directory.
async fn api_list_directory(path: &Path) -> Response {
    match tokio::fs::metadata(path).await {
        Ok(meta) => {
            if !meta.is_dir() {
                return error(
                    StatusCode::BAD_REQUEST,
                    "The '?list' parameter can only be used on directories.",
                );
            }
            match tokio::fs::read_dir(path).await {
                Ok(mut entries) => {
                    let mut files = Vec::new();
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        if let Ok(meta) = entry.metadata().await {
                            let modified_time: DateTime<Utc> = meta
                                .modified()
                                .unwrap_or(std::time::SystemTime::now())
                                .into();
                            files.push(json!({
                                "name": entry.file_name().to_string_lossy(),
                                "is_dir": meta.is_dir(),
                                "modified_time": modified_time.to_rfc3339(),
                            }));
                        }
                    }
                    success(Some(json!(files)))
                }
                Err(e) => error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read directory: {}", e),
                ),
            }
        }
        Err(_) => error(StatusCode::NOT_FOUND, "Resource not found."),
    }
}
