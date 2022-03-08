use crate::error;
use crate::error::Result;
use crate::utils::config::{get_config_path, Configurations};
use clap::{Args, Subcommand};
use colored::*;
use log;
use std::fs;
use std::path::Path;

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
            log::info!(
                "{} configuration file at {}.",
                "Initialised".cyan(),
                config_path.display()
            );
            Ok(())
        }
        ConfigurationSubcommands::View => {
            log::debug!(
                "Loaded configuration from {}",
                String::from(config_path.to_str().unwrap()).bold()
            );

            let _ = match view(&config_path) {
                Ok(config) => config,
                Err(e) => {
                    log::error!(
                        "Cannot not read configuration file. Try initializing configuration first."
                    );
                    return Err(e);
                }
            };
            return Ok(());
        }
    }
}

fn view(path: &Path) -> Result<()> {
    let output = fs::read_to_string(path)?;
    log::info!("{}", output);
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
