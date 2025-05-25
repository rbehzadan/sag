use crate::config::AuthConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteConfig {
    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub target: String,

    #[serde(default)]
    pub auth: AuthConfig,
}
