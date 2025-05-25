use crate::config::AppConfig;
use anyhow::{Context, Result, anyhow};
use config::{Config, Environment, File};
use once_cell::sync::Lazy;
use std::path::Path;

// pub static ENV_PREFIX = "APP";
pub static ENV_PREFIX: Lazy<String> = Lazy::new(|| env!("CARGO_PKG_NAME").to_uppercase());

pub fn load_config(path_opt: Option<&Path>) -> Result<AppConfig> {
    let mut builder = Config::builder();

    if let Some(path) = path_opt {
        if !path.exists() {
            return Err(anyhow!("Config file does not exist: {}", path.display()));
        }

        let path_str = path
            .to_str()
            .ok_or_else(|| anyhow!("Invalid UTF-8 in path"))?;
        builder = builder.add_source(File::with_name(path_str).required(true));
    } else {
        builder = builder.add_source(File::with_name("config.yaml").required(false));
    }

    builder = builder.add_source(Environment::with_prefix(&ENV_PREFIX).separator("__"));

    let config = builder.build().context("Failed to build configuration")?;

    config
        .try_deserialize()
        .context("Failed to deserialize AppConfig")
}
