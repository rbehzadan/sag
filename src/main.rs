// src/main.rs
mod config;
mod logging;

use anyhow::Result;
use clap::{Arg, Command};
use std::path::PathBuf;
#[allow(unused_imports)]
use tracing::{debug, error, info};

fn main() -> Result<()> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to the config file")
                .value_parser(clap::value_parser!(PathBuf))
                .required(false),
        )
        .get_matches();

    let config_path_opt = matches.get_one::<PathBuf>("config");
    let mut app_config = config::load_config(config_path_opt.map(|p| p.as_ref()))?;

    if app_config.debug {
        app_config.logging.level = "debug".to_string();
    }

    // Initialize logging early
    if let Err(e) = logging::init_logging(&app_config.logging) {
        eprintln!("Failed to initialize logging: {}", e);
        return Err(e);
    }

    info!("Starting Simple API Gateway (SAG)");
    debug!("Configuration loaded");
    debug!("{:#?}", app_config);

    if app_config.debug {
        debug!("Debug mode enabled");
        debug!("Server config: {:?}", app_config.server);
        for (i, route) in app_config.routes.iter().enumerate() {
            debug!(
                "Route {}: {} -> {} (methods: {:?})",
                i, route.path, route.target, route.methods
            );
        }
    }

    Ok(())
}
