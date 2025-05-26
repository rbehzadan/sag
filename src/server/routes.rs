use crate::config::RouteConfig;
use crate::server::{
    error::ServerError,
    matcher::{RouteMatch, RouteMatcher},
    proxy::ProxyClient,
};
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
    pub matcher: Arc<RouteMatcher>,
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

    // Find matching route using the new matcher
    let route_match = state
        .matcher
        .find_match(path)
        .ok_or(ServerError::RouteNotFound)?;

    // Check if method is allowed
    if !is_method_allowed(&route_match.route.methods, method) {
        return Err(ServerError::RouteNotFound);
    }

    debug!(
        "Matched route: {} -> {} (params: {:?})",
        route_match.route.path, route_match.route.target, route_match.params
    );

    // TODO: Use route_match.params for path parameter substitution in target URL
    state
        .proxy_client
        .proxy_request(request, &route_match.route.target)
        .await
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
}
