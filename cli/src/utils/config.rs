use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub auth_token: Option<String>,
}

impl Configuration {
    fn init(token: Option<String>) -> Self {
        Self {
            endpoint: String::from("https://agent.ssi.community"),
            api_key: None,
            auth_token: token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configurations {
    pub configurations: BTreeMap<String, Configuration>,
}

impl Configurations {
    pub fn init(token: Option<String>) -> Self {
        let mut configurations = BTreeMap::<String, Configuration>::new();
        configurations.insert(String::from("default"), Configuration::init(token));
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
