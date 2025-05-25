use crate::logging::config::{ConsoleConfig, LogFormat};
use anyhow::Result;
use tracing::Level;
use tracing_subscriber::{
    EnvFilter, Layer,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init_console_logging(
    level: &str,
    format: &LogFormat,
    console_config: &ConsoleConfig,
) -> Result<()> {
    if !console_config.enabled {
        return Ok(());
    }

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    let console_layer = match format {
        LogFormat::Pretty => fmt::layer()
            .pretty()
            .with_ansi(console_config.colors)
            .with_span_events(FmtSpan::CLOSE)
            .with_filter(env_filter)
            .boxed(),

        LogFormat::Compact => fmt::layer()
            .compact()
            .with_ansi(console_config.colors)
            .with_filter(env_filter)
            .boxed(),

        LogFormat::Json => fmt::layer()
            .json()
            .with_ansi(false) // JSON shouldn't have colors
            .with_filter(env_filter)
            .boxed(),
    };

    tracing_subscriber::registry().with(console_layer).init();

    Ok(())
}

#[allow(dead_code)]
// Helper function to parse log level string
pub fn parse_log_level(level: &str) -> Level {
    match level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" | "warning" => Level::WARN,
        "error" => Level::ERROR,
        _ => {
            eprintln!("Unknown log level '{}', defaulting to INFO", level);
            Level::INFO
        }
    }
}
