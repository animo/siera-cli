use std::path::Path;

use crate::cli::{Cli, Commands};
use crate::error::Result;
use crate::modules::{
    connections::parse_connection_args, features::parse_features_args, schema::parse_schema_args,
};
use crate::utils::{config::get_value_from_config, logger::Log};
use agent::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use clap::Parser;

pub async fn register() -> Result<()> {
    let cli = Cli::parse();

    let logger = Log {
        should_copy: cli.copy,
        suppress_output: cli.suppress,
    };

    let home = env!("HOME");
    let default_config_path = Path::new(home).join(".config/aries-cli/config.ini");
    let config_path = cli.config.unwrap_or(default_config_path);
    let environment = cli.environment;

    let endpoint_from_config = get_value_from_config(&config_path, &environment, "endpoint");
    let api_key_from_config = get_value_from_config(&config_path, &environment, "api_key");

    let endpoint = cli.endpoint.unwrap_or(endpoint_from_config?);

    let api_key = cli.api_key.or(api_key_from_config.ok());

    let version = CloudAgentPythonVersion::ZeroSixZero;

    let agent = CloudAgentPython::new(endpoint, api_key, version).await?;

    match &cli.commands {
        Commands::Connections(options) => {
            parse_connection_args(&options.commands, agent, logger).await
        }
        Commands::Schema(_) => parse_schema_args(agent, logger).await,
        Commands::Features(_) => parse_features_args(agent, logger).await,
    }?;

    Ok(())
}
