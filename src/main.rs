//! A toolkit to interact with Aries instances for data manipulation

#[macro_use]
extern crate clap;
use clap::App;
mod parse;
mod typing;

/// Initializes the application
fn main() {
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

        // test print
        println!("{}", json.name);
    }
}
