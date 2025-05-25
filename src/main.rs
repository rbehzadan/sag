// src/main.rs
mod config;

use anyhow::Result;
use clap::{Arg, Command};
use std::path::PathBuf;

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
    let app_config = config::load_config(config_path_opt.map(|p| p.as_ref()))?;

    println!("{:#?}", app_config);

    let route1 = config::RouteConfig::default();
    println!("{:?}", route1.methods);

    Ok(())
}
