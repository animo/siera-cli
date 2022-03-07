use std::fs;
use std::path::Path;

use clap::Args;
use log::info;

use crate::error;
use crate::error::Result;
use crate::utils::config::{get_config_path, Configuration};
use colored::*;
use clap::{Args, Subcommand};

use crate::error;
use crate::error::Result;
use crate::utils::config::{get_config_path, Configurations};
use crate::utils::logger::Log;

#[derive(Args)]
pub struct ConfigurationOptions {
    #[clap(subcommand)]
    pub commands: ConfigurationSubcommands,
}

#[derive(Subcommand, Debug)]
pub enum ConfigurationSubcommands {
    Initialize,
    View,
}

pub async fn parse_configuration_args(options: &ConfigurationOptions) -> Result<()> {
    let config_path = get_config_path()?;
    match options.commands {
        ConfigurationSubcommands::Initialize => {
            initialize(&config_path)?;
            logger.log("Initialized the configuration!");
            return Ok(());
        }
        ConfigurationSubcommands::View => view(&config_path, logger),
    }
}

fn view(path: &Path) -> Result<()> {
    let output = fs::read_to_string(path)?;
    info!("{}", output);
    Ok(())
}

fn initialize(path: &Path) -> Result<()> {
    // Check if the path exists and stop so we do not override the existing configuration file
    if path.exists() {
        return Err(error::Error::ConfigExists.into());
    }

    // Get the directories
    let prefix = path.parent().unwrap();

    // create all the required directories
    fs::create_dir_all(prefix)?;

    // Create the configuration file
    fs::File::create(&path)?;

    // Write the default configuration to the file
    fs::write(path, serde_yaml::to_string(&Configurations::default())?)?;

    Ok(())
}
