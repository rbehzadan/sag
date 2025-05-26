use crate::config::{MatchType, RouteConfig};
use regex::Regex;
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct RouteMatch {
    pub route: RouteConfig,
    pub params: HashMap<String, String>,
}

pub struct RouteMatcher {
    routes: Vec<CompiledRoute>,
}

#[derive(Debug, Clone)]
struct CompiledRoute {
    config: RouteConfig,
    regex: Option<Regex>,
    param_names: Vec<String>,
}

impl RouteMatcher {
    pub fn new(routes: Vec<RouteConfig>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut compiled_routes = Vec::new();

        for route in routes {
            let compiled = match route.match_type {
                MatchType::Exact => CompiledRoute {
                    config: route,
                    regex: None,
                    param_names: Vec::new(),
                },
                MatchType::Wildcard => {
                    let (regex, params) = compile_wildcard_pattern(&route.path)?;
                    CompiledRoute {
                        config: route,
                        regex: Some(regex),
                        param_names: params,
                    }
                }
                MatchType::Regex => {
                    let regex = Regex::new(&route.path)?;
                    CompiledRoute {
                        config: route,
                        regex: Some(regex),
                        param_names: Vec::new(),
                    }
                }
                MatchType::Prefix => CompiledRoute {
                    config: route,
                    regex: None,
                    param_names: Vec::new(),
                },
            };
            compiled_routes.push(compiled);
        }

        Ok(Self {
            routes: compiled_routes,
        })
    }

    pub fn find_match(&self, path: &str) -> Option<RouteMatch> {
        for route in &self.routes {
            if let Some(params) = self.matches_route(route, path) {
                debug!(
                    "Route matched: {} -> {} (type: {:?})",
                    route.config.path, route.config.target, route.config.match_type
                );
                return Some(RouteMatch {
                    route: route.config.clone(),
                    params,
                });
            }
        }
        None
    }

    fn matches_route(&self, route: &CompiledRoute, path: &str) -> Option<HashMap<String, String>> {
        match route.config.match_type {
            MatchType::Exact => {
                if route.config.path.is_empty() {
                    // Empty route path matches root
                    if path == "/" {
                        Some(HashMap::new())
                    } else {
                        None
                    }
                } else if route.config.path == path {
                    Some(HashMap::new())
                } else {
                    None
                }
            }
            MatchType::Prefix => {
                if path.starts_with(&route.config.path) {
                    Some(HashMap::new())
                } else {
                    None
                }
            }
            MatchType::Wildcard | MatchType::Regex => {
                if let Some(ref regex) = route.regex {
                    if let Some(captures) = regex.captures(path) {
                        let mut params = HashMap::new();

                        // Extract named parameters
                        for (i, param_name) in route.param_names.iter().enumerate() {
                            if let Some(capture) = captures.get(i + 1) {
                                params.insert(param_name.clone(), capture.as_str().to_string());
                            }
                        }

                        Some(params)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

fn compile_wildcard_pattern(pattern: &str) -> Result<(Regex, Vec<String>), regex::Error> {
    let mut regex_pattern = String::new();
    let mut param_names = Vec::new();
    let mut chars = pattern.chars().peekable();

    regex_pattern.push('^');

    while let Some(ch) = chars.next() {
        match ch {
            '*' => {
                if chars.peek() == Some(&'*') {
                    // ** matches multiple path segments
                    chars.next(); // consume second *
                    regex_pattern.push_str(".*");
                } else {
                    // * matches single path segment (not including /)
                    regex_pattern.push_str("[^/]*");
                }
            }
            '{' => {
                // Extract parameter name
                let mut param_name = String::new();
                while let Some(ch) = chars.next() {
                    if ch == '}' {
                        break;
                    }
                    param_name.push(ch);
                }

                if !param_name.is_empty() {
                    param_names.push(param_name);
                    regex_pattern.push_str("([^/]+)"); // Capture group for parameter
                }
            }
            // Escape special regex characters
            '.' | '^' | '$' | '(' | ')' | '[' | ']' | '|' | '+' | '?' | '\\' => {
                regex_pattern.push('\\');
                regex_pattern.push(ch);
            }
            _ => {
                regex_pattern.push(ch);
            }
        }
    }

    regex_pattern.push('$');

    debug!(
        "Compiled wildcard pattern '{}' to regex: '{}'",
        pattern, regex_pattern
    );

    let regex = Regex::new(&regex_pattern)?;
    Ok((regex, param_names))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MatchType;

    fn create_route(path: &str, match_type: MatchType) -> RouteConfig {
        RouteConfig {
            path: path.to_string(),
            target: "http://example.com".to_string(),
            methods: vec!["GET".to_string()],
            auth: Default::default(),
            match_type,
        }
    }

    #[test]
    fn test_exact_matching() {
        let routes = vec![create_route("/api/v1", MatchType::Exact)];
        let matcher = RouteMatcher::new(routes).unwrap();

        assert!(matcher.find_match("/api/v1").is_some());
        assert!(matcher.find_match("/api/v2").is_none());
        assert!(matcher.find_match("/api/v1/users").is_none());
    }

    #[test]
    fn test_wildcard_matching() {
        let routes = vec![
            create_route("/api/*/users", MatchType::Wildcard),
            create_route("/files/**", MatchType::Wildcard),
        ];
        let matcher = RouteMatcher::new(routes).unwrap();

        // Single wildcard
        assert!(matcher.find_match("/api/v1/users").is_some());
        assert!(matcher.find_match("/api/v2/users").is_some());
        assert!(matcher.find_match("/api/users").is_none()); // * must match something

        // Double wildcard
        assert!(matcher.find_match("/files/docs/readme.txt").is_some());
        assert!(matcher.find_match("/files/").is_some());
        assert!(matcher.find_match("/files").is_some());
    }

    #[test]
    fn test_parameter_extraction() {
        let routes = vec![create_route(
            "/users/{id}/posts/{post_id}",
            MatchType::Wildcard,
        )];
        let matcher = RouteMatcher::new(routes).unwrap();

        if let Some(route_match) = matcher.find_match("/users/123/posts/456") {
            assert_eq!(route_match.params.get("id"), Some(&"123".to_string()));
            assert_eq!(route_match.params.get("post_id"), Some(&"456".to_string()));
        } else {
            panic!("Route should match");
        }
    }

    #[test]
    fn test_prefix_matching() {
        let routes = vec![create_route("/api", MatchType::Prefix)];
        let matcher = RouteMatcher::new(routes).unwrap();

        assert!(matcher.find_match("/api").is_some());
        assert!(matcher.find_match("/api/v1").is_some());
        assert!(matcher.find_match("/api/v1/users").is_some());
        assert!(matcher.find_match("/different").is_none());
    }

    #[test]
    fn test_regex_matching() {
        let routes = vec![create_route(r"^/api/v\d+/users$", MatchType::Regex)];
        let matcher = RouteMatcher::new(routes).unwrap();

        assert!(matcher.find_match("/api/v1/users").is_some());
        assert!(matcher.find_match("/api/v2/users").is_some());
        assert!(matcher.find_match("/api/v10/users").is_some());
        assert!(matcher.find_match("/api/vX/users").is_none());
        assert!(matcher.find_match("/api/v1/users/123").is_none());
    }
}
