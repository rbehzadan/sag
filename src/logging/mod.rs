pub mod config;
#[allow(unused_imports)]
pub use config::{ConsoleConfig, LogFormat, LoggingConfig};

pub mod console;

use anyhow::Result;

/// Initialize logging based on the provided configuration
pub fn init_logging(logging_config: &LoggingConfig) -> Result<()> {
    console::init_console_logging(
        &logging_config.level,
        &logging_config.format,
        &logging_config.console,
    )?;

    tracing::info!("Logging initialized with level: {}", logging_config.level);

    Ok(())
}
