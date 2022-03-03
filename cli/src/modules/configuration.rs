use std::env;
use std::path::Path;
use std::{fmt, fs};

use clap::Args;

use crate::error;
use crate::error::Result;
use crate::utils::logger::Log;

#[derive(Args)]
pub struct ConfigurationOptions {
    #[clap(short, long, conflicts_with = "view")]
    initialize: bool,

    #[clap(short, long, conflicts_with = "initialize")]
    view: bool,
}

struct ConfigurationEnvironment {
    environment: String,
    endpoint: String,
    api_key: Option<String>,
}

impl fmt::Display for ConfigurationEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]\nendpoint={}{}",
            self.environment,
            self.endpoint,
            self.api_key
                .as_ref()
                .map(|val| format!("\napi_key={val}"))
                .unwrap_or_else(|| "".to_string())
        )
    }
}

// TODO: we should implement `from` so we can use todo and have a cleaner api
pub async fn parse_configuration_args(options: &ConfigurationOptions, logger: Log) -> Result<()> {
    let default_config_path = if cfg!(windows) {
        let home = "%AppData%";
        Path::new(home).join("aries-cli/config.ini")
    } else {
        let home = env!("HOME");
        Path::new(home).join(".config/aries-cli/config.ini")
    };
    if options.initialise {
        initialise(&default_config_path)?;
        logger.log("Initialised the configuration!");
        return Ok(());
    }
    if options.view {
        return view(&default_config_path, logger);
    }

    Err(error::Error::NoFlagSupplied("configuration".to_string()).into())
}

fn view(path: &Path, logger: Log) -> Result<()> {
    let output = fs::read_to_string(path)?;
    logger.log(output);
    Ok(())
}

fn initialise(path: &Path) -> Result<()> {
    let config = ConfigurationEnvironment {
        environment: "Default".to_string(),
        endpoint: "https://agent.community.animo.id".to_string(),
        api_key: None,
    };

    if path.exists() {
        return Err(error::Error::ConfigExists.into());
    }

    // Get the directories
    let prefix = path.parent().unwrap();

    // create all the required directories
    fs::create_dir_all(prefix)?;

    // Create the configuration file
    fs::File::create(&path)?;

    // Write the default configuration to the file
    fs::write(path, config.to_string())?;

    Ok(())
}
