use super::{error, typing};
use std::fs;

// Parses a json file from a relative path
pub fn parse_json_from_path(path: &str) -> typing::Config {
    // Gets the content of the configuration file
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => error::throw(error::Error::InvalidRelativePath),
    };

    // Parse the file to a `Config` type
    let value: typing::Config = match serde_json::from_str(&file) {
        Ok(value) => value,
        Err(_) => error::throw(error::Error::InvalidConfigFile),
    };

    value
}
