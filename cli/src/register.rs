use std::path::{Path, PathBuf};

use crate::cli::{Cli, Commands};
use crate::error::{self, Result};
use crate::debug;
use crate::modules::configuration::parse_configuration_args;
use crate::modules::credential_definition::parse_credential_definition_args;
use crate::modules::credentials::parse_credentials_args;
use crate::modules::message::parse_message_args;
use crate::modules::{
    connections::parse_connection_args, features::parse_features_args, schema::parse_schema_args,
};
use crate::utils::{config::get_value_from_config, logger};
use agent_controller::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use clap::Parser;
use log::LevelFilter;

pub async fn register() -> Result<()> {
    let cli = Cli::parse();
    let level = if cli.quiet {
        LevelFilter::Off
    } else {
        LevelFilter::max()
    };
    logger::init(level);

    debug!("Parsed CLI options and initialized logger");

    match &cli.commands {
        Commands::Configuration(options) => parse_configuration_args(options).await,
        Commands::Schema(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_schema_args(options, agent).await
        }
        Commands::Features(_) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_features_args(agent).await
        }
        Commands::Message(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_message_args(options, agent).await
        }
        Commands::CredentialDefinition(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_credential_definition_args(options, agent).await
        }
        Commands::Connections(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_connection_args(options, agent, cli.copy).await
        }
        Commands::Credentials(options) => {
            let agent =
                initialise_agent_from_cli(cli.config, cli.environment, cli.endpoint, cli.api_key)?;
            parse_credentials_args(&options.commands, agent).await
        }
    }?;

    debug!("Successfully executed command");
    Ok(())
}

fn initialise_agent_from_cli(
    config: Option<PathBuf>,
    environment: String,
    endpoint: Option<String>,
    api_key: Option<String>,
) -> Result<CloudAgentPython> {
    let home = env!("HOME");
    let default_config_path = Path::new(home).join(".config/aries-cli/config.ini");
    let config_path = config.unwrap_or(default_config_path);
    let environment = environment;

    // We cannot infer type of error here with `.into()` as we are async
    let endpoint_from_config = get_value_from_config(&config_path, &environment, "endpoint")
        .map_err(|_| Box::new(error::Error::NoEndpointSupplied) as Box<dyn std::error::Error>);
    let api_key_from_config = get_value_from_config(&config_path, &environment, "api_key");

    let endpoint = match endpoint {
        Some(e) => e,
        None => endpoint_from_config?,
    };

    let api_key = api_key.or_else(|| api_key_from_config.ok());

    let version = CloudAgentPythonVersion::ZeroSixZero;

    let agent = CloudAgentPython::new(endpoint, api_key, version)?;
    Ok(agent)
}
