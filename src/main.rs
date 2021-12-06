//! A toolkit to interact with Aries instances for data manipulation
//! run `adg run` to run the example script

#[macro_use]
extern crate clap;
use clap::App;

mod agent;
mod cli;
mod error;
mod typing;
mod utils;

/// Initializes the application
#[tokio::main]
async fn main() {
    // Load the yaml file containing the cli setup
    let yaml = load_yaml!("../cli.yaml");

    // Get all the supplied flags and values
    let matches = App::from_yaml(yaml).get_matches();

    // Matches the `agent` subcommand
    if let Some(matches_agent) = matches.subcommand_matches("agent") {
        let should_create_invitation = matches_agent.is_present("create-invitation");

        if should_create_invitation {
            let config = matches_agent.value_of("config").unwrap_or("default.json");

            // JSON object containing the config
            let json: typing::Config = utils::parse::parse_json_from_path(config);

            cli::agent::run(json).await;
        }
    }
}
