use crate::config::AuthConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteConfig {
    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub target: String,

    #[serde(default = "default_methods")]
    pub methods: Vec<String>,

    #[serde(default)]
    pub auth: AuthConfig,

    #[serde(default)]
    pub match_type: MatchType,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum MatchType {
    #[default]
    Exact, // Exact path matching (default)
    Wildcard, // Wildcard matching with * and **
    Regex,    // Regex pattern matching
    Prefix,   // Prefix matching (starts with)
}

fn default_methods() -> Vec<String> {
    vec!["GET".to_string(), "POST".to_string()]
}

impl Default for RouteConfig {
    fn default() -> Self {
        Self {
            path: String::new(),
            target: String::new(),
            methods: default_methods(),
            auth: AuthConfig::default(),
            match_type: MatchType::default(),
        }
    }
}
