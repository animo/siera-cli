use clap::Parser;
use cloudagent_python::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use colored::*;
use log::debug;
use log::LevelFilter;
use std::path::PathBuf;

use crate::cli::{Cli, Commands};
use crate::error::{Error, Result};
use crate::modules::automation::parse_automation_args;
use crate::modules::basic_message::parse_basic_message_args;
use crate::modules::configuration::parse_configuration_args;
use crate::modules::credential::parse_credentials_args;
use crate::modules::credential_definition::parse_credential_definition_args;
use crate::modules::proof::parse_proof_args;
use crate::modules::{
    connection::parse_connection_args, feature::parse_features_args, schema::parse_schema_args,
};
use crate::utils::config::{get_config_from_path, get_config_path};
use crate::utils::logger;

/// Register the subcommands on the cli
pub async fn register() -> Result<()> {
    let cli = Cli::parse();
    let level = if cli.quiet {
        LevelFilter::Warn
    } else {
        match cli.verbose {
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            3.. => LevelFilter::Trace,
            _ => LevelFilter::Warn,
        }
    };
    logger::init(level, cli.copy);

    debug!("Parsed CLI options and initialized logger");

    match &cli.commands {
        Commands::Configuration(options) => parse_configuration_args(options).await,
        Commands::Schema(options) => {
            let agent = initialize_agent_from_cli(
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;
            parse_schema_args(options, agent).await
        }
        Commands::Feature(_) => {
            let agent = initialize_agent_from_cli(
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;
            parse_features_args(agent).await
        }
        Commands::Message(options) => {
            let agent = initialize_agent_from_cli(
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;
            parse_basic_message_args(options, agent).await
        }
        Commands::CredentialDefinition(options) => {
            let agent = initialize_agent_from_cli(
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;
            parse_credential_definition_args(options, agent).await
        }
        Commands::Connection(options) => {
            let agent = initialize_agent_from_cli(
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;
            parse_connection_args(options, agent).await
        }
        Commands::Credential(options) => {
            let agent = initialize_agent_from_cli(
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;
            parse_credentials_args(&options.commands, agent).await
        }
        Commands::Proof(options) => {
            let agent = initialize_agent_from_cli(
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;
            parse_proof_args(&options.commands, agent).await
        }
        Commands::Automate(options) => {
            let agent = initialize_agent_from_cli(
                cli.config,
                cli.environment,
                cli.agent_url,
                cli.api_key,
                cli.token,
            )?;
            parse_automation_args(options, agent).await
        }
    }?;

    debug!("{} executed command", "Successfully".green());
    Ok(())
}

/// Initialize any agent from the cli
fn initialize_agent_from_cli(
    config: Option<PathBuf>,
    environment: String,
    agent_url: Option<String>,
    api_key: Option<String>,
    auth_token: Option<String>,
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

    let (agent_url, api_key, auth_token) = match config_path {
        Some(cp) => {
            let configurations = get_config_from_path(&cp)?;
            let configuration = configurations
                .configurations
                .get_key_value(&environment)
                .ok_or(Error::InvalidEnvironment(environment))?;
            let agent_url = agent_url.unwrap_or_else(|| configuration.1.endpoint.to_owned());
            let api_key = api_key.or_else(|| configuration.1.api_key.to_owned());
            let auth_token = auth_token.or_else(|| configuration.1.auth_token.to_owned());
            (agent_url, api_key, auth_token)
        }
        None => {
            let agent_url = agent_url.ok_or(Error::NoAgentURLSupplied)?;
            (agent_url, api_key, auth_token)
        }
    };

    let version = CloudAgentPythonVersion::ZeroSevenThree;
    Ok(CloudAgentPython::new(
        agent_url, api_key, auth_token, version,
    ))
}
