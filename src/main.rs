#[macro_use]
extern crate clap;
use clap::App;

/// Initializes the application
fn main() {
    // Load the yaml file containing the cli setup
    let yaml = load_yaml!("../cli.yaml");

    // Get all the supplied flags and values
    let matches = App::from_yaml(yaml).get_matches();

    // Load the configuration file
    let _config = matches.value_of("config").unwrap_or("default.conf");
}
