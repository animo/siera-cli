//! A toolkit to interact with Aries instances for data manipulation
//! run `adg run` to run the example script

#[macro_use]
extern crate clap;
use clap::App;

mod agent;
mod error;
mod mediator;
mod run;
mod typing;
mod utils;

/// Initializes the application
#[tokio::main]
async fn main() {
    // Load the yaml file containing the cli setup
    let yaml = load_yaml!("../cli.yaml");

    // Get all the supplied flags and values
    let matches = App::from_yaml(yaml).get_matches();

    // Matches the `run` subcommand
    if let Some(matches_run) = matches.subcommand_matches("run") {
        // Path of the config file
        let config = matches_run.value_of("config").unwrap_or("default.json");

        // JSON object containing the config
        let json: typing::Config = utils::parse::parse_json_from_path(config);

        run::cli::run(json).await;
    }

    // Matches the `mediator` subcommand
    if let Some(matches_mediator) = matches.subcommand_matches("mediator") {
        let should_create_invitation = matches_mediator.is_present("create-invitation");

        if should_create_invitation {
            let config = matches_mediator
                .value_of("config")
                .unwrap_or("default.json");

            // JSON object containing the config
            let json: typing::Config = utils::parse::parse_json_from_path(config);

            mediator::cli::run(json).await;
        }
    }
}
