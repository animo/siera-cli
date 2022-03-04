use std::fmt;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub endpoint: String,
    pub api_key: Option<String>,
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "  - name: {}\n    endpoint: {}\n{}",
            self.name,
            self.endpoint,
            self.api_key
                .as_ref()
                .map(|val| format!("    apiKey: {}\n", val))
                .unwrap_or_else(|| "".to_string())
        )
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            name: String::from("default"),
            endpoint: String::from("https://agent.community.animo.id"),
            api_key: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configurations {
    pub configurations: Vec<Configuration>,
}

pub fn get_config_from_path(config_path: PathBuf) -> Result<Configurations> {
    let out: Result<String> =
        std::fs::read_to_string(config_path).map_err(|_| Error::InvalidConfigurationPath.into());
    serde_yaml::from_str::<Configurations>(&out?)
        .map_err(|_| Error::InvalidConfigurationStructure.into())
}

pub fn get_config_path() -> Result<PathBuf> {
    if cfg!(windows) {
        let home = "C:\\Program Files\\Common Files";
        Ok(Path::new(home).join("aries-cli\\config.yaml"))
    } else if cfg!(unix) {
        let home = option_env!("HOME").ok_or(Error::HomeNotFound);
        Ok(Path::new(&home?).join(".config/aries-cli/config.yaml"))
    } else {
        Err(Error::OsUnknown.into())
    }
}
