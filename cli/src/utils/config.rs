use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub endpoint: String,
    pub api_key: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            endpoint: String::from("https://agent.community.animo.id"),
            api_key: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configurations {
    pub configurations: BTreeMap<String, Configuration>,
}

impl Default for Configurations {
    fn default() -> Self {
        let mut configurations = BTreeMap::<String, Configuration>::new();
        configurations.insert(String::from("default"), Configuration::default());
        Self { configurations }
    }
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
