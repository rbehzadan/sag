pub mod error;
pub mod proxy;
pub mod routes;

use crate::config::AppConfig;
use anyhow::Result;
use axum::{
    Router,
    routing::{any, get},
};
use routes::{AppState, handle_request, health_check};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing::{error, info};

pub async fn start_server(config: AppConfig) -> Result<()> {
    let addr = SocketAddr::new(config.server.host, config.server.port);

    info!("Starting server on {}", addr);
    info!("Configured routes: {}", config.routes.len());

    // Log configured routes
    for (i, route) in config.routes.iter().enumerate() {
        info!(
            "Route {}: {} -> {} (methods: {:?})",
            i,
            if route.path.is_empty() {
                "/"
            } else {
                &route.path
            },
            route.target,
            route.methods
        );
    }

    // Create shared state
    let state = AppState {
        routes: config.routes,
        proxy_client: Arc::new(proxy::ProxyClient::new()),
        debug: config.debug,
    };

    // Build the router
    let app = Router::new()
        .route("/health", get(health_check))
        .fallback(any(handle_request))
        .with_state(state);

    // Create listener
    let listener = TcpListener::bind(&addr).await.map_err(|e| {
        error!("Failed to bind to {}: {}", addr, e);
        e
    })?;

    info!("Server listening on {}", addr);

    // Start the server
    axum::serve(listener, app).await.map_err(|e| {
        error!("Server error: {}", e);
        e.into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{RouteConfig, ServerConfig};
    use std::net::{IpAddr, Ipv4Addr};

    fn create_test_config() -> AppConfig {
        AppConfig {
            server: ServerConfig {
                host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                port: 8080,
                max_connections: 1024,
            },
            routes: vec![RouteConfig {
                path: "/api/v1".to_string(),
                target: "http://localhost:3000".to_string(),
                methods: vec!["GET".to_string(), "POST".to_string()],
                auth: Default::default(),
            }],
            logging: Default::default(),
            debug: false,
        }
    }

    #[test]
    fn test_config_creation() {
        let config = create_test_config();
        assert_eq!(config.routes.len(), 1);
        assert_eq!(config.routes[0].path, "/api/v1");
    }
}
