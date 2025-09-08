/* src/response.rs */

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicSuccessResponse {
    status: String,
    data: serde_json::Value,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicErrorResponse {
    status: String,
    message: String,
    timestamp: String,
}

// 200
pub fn success(data: Option<serde_json::Value>) -> Response {
    let response = PublicSuccessResponse {
        status: "Success".to_string(),
        data: data.unwrap_or_else(|| json!({})),
        timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    };
    (StatusCode::OK, Json(response)).into_response()
}

// 4xx, 5xx
pub fn error(status: StatusCode, message: impl Into<String>) -> Response {
    let response = PublicErrorResponse {
        status: "Error".to_string(),
        message: message.into(),
        timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    };
    (status, Json(response)).into_response()
}
