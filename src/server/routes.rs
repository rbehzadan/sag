use crate::config::RouteConfig;
use crate::server::{error::ServerError, proxy::ProxyClient};
use axum::{
    body::Body,
    extract::{Request, State},
    http::Method,
    response::Response,
};
use std::sync::Arc;
use tracing::debug;

#[derive(Clone)]
pub struct AppState {
    pub routes: Vec<RouteConfig>,
    pub proxy_client: Arc<ProxyClient>,
    #[allow(dead_code)]
    pub debug: bool,
}

pub async fn handle_request(
    State(state): State<AppState>,
    request: Request,
) -> Result<Response<Body>, ServerError> {
    let method = request.method();
    let path = request.uri().path();

    debug!("Incoming request: {} {}", method, path);

    // Find matching route
    let route =
        find_matching_route(&state.routes, method, path).ok_or(ServerError::RouteNotFound)?;

    debug!("Matched route: {} -> {}", route.path, route.target);

    // For now, we'll implement simple exact path matching
    // Later we'll add wildcard and regex support
    state
        .proxy_client
        .proxy_request(request, &route.target)
        .await
}

fn find_matching_route<'a>(
    routes: &'a [RouteConfig],
    method: &Method,
    path: &str,
) -> Option<&'a RouteConfig> {
    for route in routes {
        // Check if method is allowed
        if !is_method_allowed(&route.methods, method) {
            continue;
        }

        // For now, do exact path matching
        // Later we'll implement wildcard and regex matching
        if matches_path(&route.path, path) {
            return Some(route);
        }
    }
    None
}

fn is_method_allowed(allowed_methods: &[String], method: &Method) -> bool {
    if allowed_methods.is_empty() {
        // If no methods specified, allow all
        return true;
    }

    let method_str = method.as_str();
    allowed_methods
        .iter()
        .any(|m| m.eq_ignore_ascii_case(method_str))
}

fn matches_path(route_path: &str, request_path: &str) -> bool {
    // For now, implement exact matching
    // Later we'll add support for:
    // - Wildcards: /api/v1/*
    // - Parameters: /api/v1/{id}
    // - Regex patterns

    if route_path.is_empty() {
        // Empty route path matches root
        return request_path == "/";
    }

    route_path == request_path
}

// Health check endpoint - not part of configured routes
pub async fn health_check() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Method;

    #[test]
    fn test_is_method_allowed() {
        let methods = vec!["GET".to_string(), "POST".to_string()];

        assert!(is_method_allowed(&methods, &Method::GET));
        assert!(is_method_allowed(&methods, &Method::POST));
        assert!(!is_method_allowed(&methods, &Method::PUT));

        // Empty methods list should allow all
        assert!(is_method_allowed(&[], &Method::DELETE));
    }

    #[test]
    fn test_matches_path() {
        assert!(matches_path("/api/v1", "/api/v1"));
        assert!(!matches_path("/api/v1", "/api/v2"));
        assert!(matches_path("", "/"));
        assert!(!matches_path("/api", "/api/v1"));
    }
}
