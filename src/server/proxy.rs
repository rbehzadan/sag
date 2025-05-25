use crate::server::error::ServerError;
use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, Uri},
    response::Response,
};
use reqwest::Client;
use tracing::{debug, error};

pub struct ProxyClient {
    client: Client,
}

impl ProxyClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none()) // Don't follow redirects automatically
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn proxy_request(
        &self,
        request: Request,
        target_base: &str,
    ) -> Result<Response<Body>, ServerError> {
        let (parts, body) = request.into_parts();

        // Build target URL
        let target_url = self.build_target_url(target_base, &parts.uri)?;
        debug!("Proxying {} {} to {}", parts.method, parts.uri, target_url);

        // Convert axum body to bytes
        let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Failed to read request body: {}", e);
                return Err(ServerError::RequestError(
                    "Failed to read request body".to_string(),
                ));
            }
        };

        // Prepare headers (filter out hop-by-hop headers and set correct Host)
        let headers = self.filter_request_headers(&parts.headers, &target_url);

        // Build the proxied request
        let mut req_builder = self
            .client
            .request(parts.method, &target_url)
            .headers(headers);

        if !body_bytes.is_empty() {
            req_builder = req_builder.body(body_bytes);
        }

        // Execute the request
        let response = match req_builder.send().await {
            Ok(resp) => {
                debug!("Proxy response status: {}", resp.status());
                debug!("Proxy response headers: {:?}", resp.headers());
                resp
            }
            Err(e) => {
                error!("Proxy request failed: {}", e);
                return Err(ServerError::ProxyError(format!("Request failed: {}", e)));
            }
        };

        // Convert response back to axum format
        self.convert_response(response).await
    }

    fn build_target_url(
        &self,
        target_base: &str,
        original_uri: &Uri,
    ) -> Result<String, ServerError> {
        let target_base = target_base.trim_end_matches('/');

        let path_and_query = original_uri
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/");

        let target_url = format!("{}{}", target_base, path_and_query);

        // Basic URL validation
        if !target_url.starts_with("http://") && !target_url.starts_with("https://") {
            return Err(ServerError::InvalidTarget(target_url));
        }

        Ok(target_url)
    }

    fn filter_request_headers(&self, headers: &HeaderMap, target_url: &str) -> HeaderMap {
        let mut filtered_headers = HeaderMap::new();

        // Headers that should not be forwarded (hop-by-hop headers)
        let skip_headers = [
            "connection",
            "upgrade",
            "proxy-authenticate",
            "proxy-authorization",
            "te",
            "trailers",
            "transfer-encoding",
            "host", // We'll set the correct host header below
        ];

        for (name, value) in headers {
            let name_str = name.as_str().to_lowercase();
            if !skip_headers.contains(&name_str.as_str()) {
                filtered_headers.insert(name.clone(), value.clone());
            } else {
                debug!("Filtering out header: {}", name);
            }
        }

        // Set the correct Host header for the target server
        if let Ok(url) = reqwest::Url::parse(target_url) {
            if let Some(host) = url.host_str() {
                let host_header = if let Some(port) = url.port() {
                    // Include port if it's not the default for the scheme
                    let default_port = match url.scheme() {
                        "https" => 443,
                        "http" => 80,
                        _ => 0,
                    };
                    if port != default_port {
                        format!("{}:{}", host, port)
                    } else {
                        host.to_string()
                    }
                } else {
                    host.to_string()
                };

                if let Ok(header_value) = reqwest::header::HeaderValue::from_str(&host_header) {
                    filtered_headers.insert(reqwest::header::HOST, header_value);
                }
            }
        }

        filtered_headers
    }

    async fn convert_response(
        &self,
        response: reqwest::Response,
    ) -> Result<Response<Body>, ServerError> {
        let status = response.status();
        let headers = response.headers().clone();

        // Get response body
        let body_bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Failed to read response body: {}", e);
                return Err(ServerError::ProxyError(
                    "Failed to read response body".to_string(),
                ));
            }
        };

        // Build axum response
        let mut response_builder = Response::builder().status(status);

        // Copy headers (filter hop-by-hop headers)
        for (name, value) in headers {
            if let Some(name) = name {
                let name_str = name.as_str().to_lowercase();
                let skip_headers = ["connection", "upgrade", "transfer-encoding"];

                if !skip_headers.contains(&name_str.as_str()) {
                    response_builder = response_builder.header(name, value);
                }
            }
        }

        let response = response_builder.body(Body::from(body_bytes)).map_err(|e| {
            error!("Failed to build response: {}", e);
            ServerError::InternalError("Failed to build response".to_string())
        })?;

        Ok(response)
    }
}

impl Default for ProxyClient {
    fn default() -> Self {
        Self::new()
    }
}
