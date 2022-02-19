mod cli;
mod error;
mod modules;
mod utils;

use agent::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use clap::StructOpt;
use cli::{Cli, Commands};
use modules::{connections::parse_connection_args, features::parse_features_args};
use utils::logger::Log;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let agent = CloudAgentPython::new(
        cli.endpoint.unwrap(),
        cli.api_key,
        CloudAgentPythonVersion::ZeroSixZero,
    );

    let logger = Log::default();

    match &cli.commands {
        Commands::Connections(options) => {
            parse_connection_args(&options.commands, agent, logger).await
        }
        Commands::Features(_) => parse_features_args().await,
    }
}
