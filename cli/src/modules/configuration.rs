use std::fs;
use std::path::Path;

use clap::Args;
use log::info;

use crate::error;
use crate::error::Result;
use crate::utils::config::{get_config_path, Configuration};
use colored::*;

#[derive(Args)]
pub struct ConfigurationOptions {
    #[clap(short, long, conflicts_with = "view")]
    initialize: bool,

    #[clap(short, long, conflicts_with = "initialize")]
    view: bool,
}

pub async fn parse_configuration_args(options: &ConfigurationOptions, logger: Log) -> Result<()> {
    let config_path = get_config_path()?;
    if options.initialize {
        initialise(&config_path)?;
        info!("{} the configuration", "Initialised".cyan());
        return Ok(());
    }
    if options.view {
        return view(&config_path, logger);
    }

    Err(error::Error::NoFlagSupplied("configuration".to_string()).into())
}

fn view(path: &Path) -> Result<()> {
    let output = fs::read_to_string(path)?;
    println!("{}", output);
    Ok(())
}

fn initialise(path: &Path) -> Result<()> {
    let config = Configuration::default();

    if path.exists() {
        return Err(error::Error::ConfigExists.into());
    }

    // Get the directories
    let prefix = path.parent().unwrap();

    // create all the required directories
    fs::create_dir_all(prefix)?;

    // Create the configuration file
    fs::File::create(&path)?;

    let initial_configuration = format!("configurations:\n{}", config);

    // Write the default configuration to the file
    fs::write(path, initial_configuration)?;

    Ok(())
}
