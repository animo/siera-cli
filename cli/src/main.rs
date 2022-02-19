mod cli;
mod modules;
mod utils;

use agent::agent_python::agent::{CloudAgentPython, CloudAgentPythonVersion};
use clap::StructOpt;
use cli::{Cli, Commands};
use modules::{connections::parse_connection_args, features::parse_features_args};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let agent = CloudAgentPython::new(
        cli.endpoint.unwrap(),
        cli.api_key,
        CloudAgentPythonVersion::ZeroSixZero,
    );

    match &cli.commands {
        Commands::Connections(options) => parse_connection_args(&options.commands, agent).await,
        Commands::Features(_) => parse_features_args().await,
    }
}
