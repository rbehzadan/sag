use crate::config::AuthConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteConfig {
    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub target: String,

    #[serde(default = "default_methods")]
    pub methods: Vec<String>,

    #[serde(default)]
    pub auth: AuthConfig,
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
        }
    }
}
