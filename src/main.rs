//! A toolkit to interact with Aries instances for data manipulation
//! run `adg run` to run the example script

#[macro_use]
extern crate clap;
use clap::App;

mod agent;
mod cli;
mod error;
mod parse;
mod typing;
mod util;

/// Initializes the application
#[tokio::main]
async fn main() {
    // Load the yaml file containing the cli setup
    let yaml = load_yaml!("../cli.yaml");

    // Get all the supplied flags and values
    let matches = App::from_yaml(yaml).get_matches();

    // Matches the `test` subcommand
    if let Some(matches) = matches.subcommand_matches("run") {
        // Path of the config file
        let config = matches.value_of("config").unwrap_or("default.json");

        // JSON object containing the config
        let json: typing::Config = parse::parse_json_from_path(config);

        cli::run(json).await;
    }
}
