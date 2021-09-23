use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
// type of the configuration file
pub struct Config {
    pub name: String, // test configuration prop
}
