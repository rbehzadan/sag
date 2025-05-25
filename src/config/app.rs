use crate::config::RouteConfig;
use crate::config::ServerConfig;
use crate::logging::LoggingConfig;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub routes: Vec<RouteConfig>,

    #[serde(default)]
    pub logging: LoggingConfig,

    #[serde(default)]
    pub debug: bool,
}
