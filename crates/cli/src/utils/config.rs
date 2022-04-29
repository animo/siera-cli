use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub auth_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub configurations: BTreeMap<String, Environment>,
}

impl Configuration {
    pub fn init(token: Option<String>) -> (String, Environment) {
        let environment = Environment {
            // Set the multi tenancy agent when an auth token is provided
            endpoint: match token {
                Some(_) => String::from("https://agent.ssi.community"),
                None => String::from("https://agent.community.animo.id"),
            },
            api_key: None,
            auth_token: token,
        };
        (String::from("default"), environment)
    }

    /// Can do inplace mutation and initialize
    pub fn add(environment: String, configuration: Environment) -> Result<()> {
        let path = get_config_path()?;
        let mut current_configuration = get_config_from_path(&path).unwrap_or(Configuration {
            configurations: BTreeMap::new(),
        });
        current_configuration
            .configurations
            .insert(environment, configuration);

        // Create path if it does not exist
        if !path.exists() {
            // Get the directories
            let prefix = path.parent().unwrap();

            // create all the required directories
            fs::create_dir_all(prefix)?;

            // Create the configuration file
            fs::File::create(&path)?;
        }

        fs::write(
            path,
            serde_yaml::to_string(&current_configuration)?.as_bytes(),
        )?;
        Ok(())
    }

    pub fn remove(environment: String) -> Result<()> {
        let path = get_config_path()?;
        let mut current_configuration =
            get_config_from_path(&path).map_err(|_| Error::EmptyConfiguration)?;
        current_configuration
            .configurations
            .remove(&environment)
            .ok_or(Error::InvalidEnvironment(environment))?;
        fs::write(
            path,
            serde_yaml::to_string(&current_configuration)?.as_bytes(),
        )?;
        Ok(())
    }
}

pub fn get_config_from_path(config_path: &Path) -> Result<Configuration> {
    let out: Result<String> =
        std::fs::read_to_string(config_path).map_err(|_| Error::InvalidConfigurationPath.into());
    serde_yaml::from_str::<Configuration>(&out?)
        .map_err(|_| Error::InvalidConfigurationStructure.into())
}

pub fn get_config_path() -> Result<PathBuf> {
    if cfg!(windows) {
        let home = "C:\\Program Files\\Common Files";
        Ok(Path::new(home).join("agent-cli\\config.yaml"))
    } else if cfg!(unix) {
        let home = match std::env::var("HOME") {
            Ok(h) => Ok(h),
            Err(_) => Err(Error::HomeNotFound),
        };
        Ok(Path::new(&home?).join(".config/agent-cli/config.yaml"))
    } else {
        Err(Error::OsUnknown.into())
    }
}
