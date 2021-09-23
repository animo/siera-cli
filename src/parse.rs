use super::typing;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    name: String,
}

// Parses a json file from a relative path
pub fn parse_json_from_path(path: &str) -> typing::Config {
    // Gets the content of the configuration file
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => panic!("Invalid relative path"),
    };

    // Parse the file to a `Config` type
    let value: typing::Config = match serde_json::from_str(&file) {
        Ok(value) => value,
        Err(_) => panic!("Invalid json structure"),
    };

    value
}
