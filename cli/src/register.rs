use agent_controller::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use clap::Parser;
use colored::*;
use log::debug;
use log::LevelFilter;
use std::path::PathBuf;

use crate::cli::{Cli, Commands};
use crate::error::{Error, Result};
use crate::modules::configuration::parse_configuration_args;
use crate::modules::credential_definition::parse_credential_definition_args;
use crate::modules::credentials::parse_credentials_args;
use crate::modules::message::parse_message_args;
use crate::modules::{
    connections::parse_connection_args, features::parse_features_args, schema::parse_schema_args,
};
use crate::utils::config::{get_config_from_path, get_config_path};
use crate::utils::logger;

pub async fn register() -> Result<()> {
    let cli = Cli::parse();
    let level = if cli.quiet {
        LevelFilter::Error
    } else if cli.raw {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    logger::init(level, cli.copy);

    debug!("Parsed CLI options and initialized logger");

    match &cli.commands {
        Commands::Configuration(options) => parse_configuration_args(options).await,
        Commands::Schema(options) => {
            let agent =
                initialize_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_schema_args(options, agent).await
        }
        Commands::Features(_) => {
            let agent =
                initialize_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_features_args(agent).await
        }
        Commands::Message(options) => {
            let agent =
                initialize_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_message_args(options, agent).await
        }
        Commands::CredentialDefinition(options) => {
            let agent =
                initialize_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_credential_definition_args(options, agent).await
        }
        Commands::Connections(options) => {
            let agent =
                initialize_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            // TODO: refactor cli.copy
            parse_connection_args(options, agent).await
        }
        Commands::Credentials(options) => {
            let agent =
                initialize_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_credentials_args(&options.commands, agent).await
        }
    }?;

    debug!("{} executed command", "Successfully".green());
    Ok(())
}

fn initialize_agent_from_cli(
    config: Option<PathBuf>,
    environment: String,
    endpoint: Option<String>,
    api_key: Option<String>,
) -> Result<CloudAgentPython> {
    let config_path = match config {
        Some(c) => Some(c),
        None => {
            let config = get_config_path();
            match config {
                Ok(c) => {
                    if c.exists() {
                        Some(c)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        }
    };

    let (endpoint, api_key) = match config_path {
        Some(cp) => {
            let configurations = get_config_from_path(cp)?;
            let configuration = configurations
                .configurations
                .get_key_value(&environment)
                .ok_or(Error::InvalidEnvironment)?;
            let endpoint = endpoint.unwrap_or_else(|| configuration.1.endpoint.to_owned());
            let api_key = api_key.or_else(|| configuration.1.api_key.to_owned());
            (endpoint, api_key)
        }
        None => {
            let endpoint = endpoint.ok_or(Error::NoEndpointSupplied)?;
            (endpoint, api_key)
        }
    };

    let version = CloudAgentPythonVersion::ZeroSixZero;
    CloudAgentPython::new(endpoint, api_key, version)
}
