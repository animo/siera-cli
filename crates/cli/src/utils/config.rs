use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// Structure for an environment in the configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    /// The agent endpoint
    pub endpoint: String,

    /// The api key that is used for authentication
    pub api_key: Option<String>,

    /// The token which is used for a multi tenancy agent
    pub auth_token: Option<String>,

    /// The cloudagent type
    pub agent: Option<String>,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let api_key = match self.api_key {
            Some(_) => "<secret>",
            None => "<none>",
        };
        let auth_token = match self.auth_token {
            Some(_) => "<secret>",
            None => "<none>",
        };

        write!(
            f,
            "(api_key: {}, auth_token: {}, agent: {:?})",
            api_key, auth_token, self.agent
        )
    }
}

/// A generic configuration used to store multiple agent configurations
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// The configurations stored in the config file
    /// key = environment name
    /// value = environment
    pub configurations: BTreeMap<String, Environment>,
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.configurations
            .iter()
            .for_each(|c| write!(f, "({}: {})", c.0, c.1).unwrap());
        Ok(())
    }
}

impl Configuration {
    /// Initialize a configuration with some default options
    pub fn init(token: Option<String>) -> (String, Environment) {
        let environment = Environment {
            // Set the multi tenancy agent when an auth token is provided
            endpoint: match token {
                Some(_) => String::from("https://agent.ssi.community"),
                None => String::from("https://agent.community.animo.id"),
            },
            api_key: None,
            agent: Some(String::from("aca-py")),
            auth_token: token,
        };
        (String::from("default"), environment)
    }

    /// Add a new agent to the configuration or update a current environment
    pub fn add(environment: String, configuration: Environment) -> Result<()> {
        let path = get_config_path()?;
        let mut current_configuration = get_config_from_path(&path).unwrap_or(Self {
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

    /// Remove an agent from the configuration
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

/// Get the confuration file when supplying the path where it is supposed to be
/// This also parser the file
pub fn get_config_from_path(config_path: &Path) -> Result<Configuration> {
    let out: Result<String> =
        std::fs::read_to_string(config_path).map_err(|_| Error::InvalidConfigurationPath.into());
    serde_yaml::from_str::<Configuration>(&out?)
        .map_err(|_| Error::InvalidConfigurationStructure.into())
}

/// Get the default configuration path
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
