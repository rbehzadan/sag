use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum ServerError {
    ProxyError(String),
    RouteNotFound,
    InvalidTarget(String),
    RequestError(String),
    InternalError(String),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::ProxyError(msg) => write!(f, "Proxy error: {}", msg),
            ServerError::RouteNotFound => write!(f, "Route not found"),
            ServerError::InvalidTarget(target) => write!(f, "Invalid target: {}", target),
            ServerError::RequestError(msg) => write!(f, "Request error: {}", msg),
            ServerError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ServerError {}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message, debug_info) = match self {
            ServerError::ProxyError(msg) => (
                StatusCode::BAD_GATEWAY,
                "Service temporarily unavailable",
                Some(msg),
            ),
            ServerError::RouteNotFound => (StatusCode::NOT_FOUND, "Not found", None),
            ServerError::InvalidTarget(target) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Configuration error",
                Some(format!("Invalid target: {}", target)),
            ),
            ServerError::RequestError(msg) => (StatusCode::BAD_REQUEST, "Bad request", Some(msg)),
            ServerError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
                Some(msg),
            ),
        };

        // For now, we'll always include debug info
        // Later we'll make this conditional based on debug mode
        let body = if let Some(debug) = debug_info {
            json!({
                "error": error_message,
                "debug": debug
            })
        } else {
            json!({
                "error": error_message
            })
        };

        (status, Json(body)).into_response()
    }
}
