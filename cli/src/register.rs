use std::path::PathBuf;

use crate::cli::{Cli, Commands};
use crate::error::{Result, Error};
use crate::modules::configuration::parse_configuration_args;
use crate::modules::credential_definition::parse_credential_definition_args;
use crate::modules::credentials::parse_credentials_args;
use crate::modules::message::parse_message_args;
use crate::modules::{
    connections::parse_connection_args, features::parse_features_args, schema::parse_schema_args,
};
use crate::utils::config::{get_config_from_path, Configuration, get_config_path};
use crate::utils::logger::Log;
use agent_controller::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use clap::Parser;

pub async fn register() -> Result<()> {
    let cli = Cli::parse();

    let logger = Log {
        should_copy: cli.copy,
        suppress_output: cli.quiet,
        debug: cli.raw,
    };

    match &cli.commands {
        Commands::Configuration(options) => parse_configuration_args(options, logger).await,
        Commands::Schema(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_schema_args(options, agent, logger).await
        }
        Commands::Features(_) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_features_args(agent, logger).await
        }
        Commands::Message(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_message_args(options, agent, logger).await
        }
        Commands::CredentialDefinition(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_credential_definition_args(options, agent, logger).await
        }
        Commands::Connections(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_connection_args(options, agent, logger).await
        }
        Commands::Credentials(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_credentials_args(&options.commands, agent, logger).await
        }
    }?;

    Ok(())
}

fn initialise_agent_from_cli(
    config: Option<PathBuf>,
    environment: String,
    endpoint: Option<String>,
    api_key: Option<String>,
) -> Result<CloudAgentPython> {
    let config_path = match config {
        Some(c) => Some(c),
        None => get_config_path().ok()
    };

    let (endpoint, api_key) = match config_path {
        Some(cp) => {
           let configurations = get_config_from_path(cp)?;
           let configuration: Configuration = configurations
               .configurations
               .into_iter()
               .find(|c| c.name == environment)
               .ok_or(Error::InvalidEnvironment)?;
           let endpoint = endpoint.unwrap_or(configuration.endpoint);
           let api_key = api_key.or(configuration.api_key);
           (endpoint, api_key)
        },
        None => {
           let endpoint = endpoint.ok_or(Error::NoEndpointSupplied)?; 
           (endpoint, api_key)
        },
    };

    let version = CloudAgentPythonVersion::ZeroSixZero;
    CloudAgentPython::new(endpoint, api_key,version)

}
