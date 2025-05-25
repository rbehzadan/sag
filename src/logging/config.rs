use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_level")]
    pub level: String,

    #[serde(default = "default_format")]
    pub format: LogFormat,

    #[serde(default)]
    pub console: ConsoleConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConsoleConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    #[serde(default = "default_colors")]
    pub colors: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum LogFormat {
    #[default]
    Pretty,
    Compact,
    Json,
}

fn default_level() -> String {
    "info".to_string()
}

fn default_format() -> LogFormat {
    LogFormat::Pretty
}

fn default_enabled() -> bool {
    true
}

fn default_colors() -> bool {
    true
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_level(),
            format: default_format(),
            console: ConsoleConfig::default(),
        }
    }
}
