mod cli;
mod error;
mod modules;
mod utils;

use std::path::Path;

use agent::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use clap::StructOpt;
use cli::{Cli, Commands};
use modules::{connections::parse_connection_args, features::parse_features_args};
use utils::{config::get_value_from_config, logger::Log};

use crate::error::{get_cli_error_string, CliError};

#[tokio::main]
async fn main() {
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

    let endpoint = cli.endpoint.unwrap_or_else(|| {
        endpoint_from_config
            .unwrap_or_else(|| logger.error(get_cli_error_string(CliError::NoEndpointSupplied)))
    });

    let api_key = cli.api_key.or(api_key_from_config);

    // TODO: Support multiple versions
    let version = CloudAgentPythonVersion::ZeroSixZero;

    let agent = CloudAgentPython::new(endpoint, api_key, version)
        .await
        .unwrap_or_else(|e| logger.error(format!("{:?}", e.to_string())));

    match &cli.commands {
        Commands::Connections(options) => {
            parse_connection_args(&options.commands, agent, logger).await
        }
        Commands::Features(_) => parse_features_args(agent, logger).await,
    }
}
