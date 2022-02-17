use super::register::ModuleWithBaseAgent;
use crate::{
    agent::agents::BaseAgent,
    error::{throw, Error},
};
use async_trait::async_trait;
use clap::ArgMatches;

/// Which action should be executed on the config
pub enum ConfigurationAction {
    /// Create a new configuration
    Create,

    /// Edit the existing configuration
    Edit,

    /// Outputs the default The configuration [Default]
    Output,
}

/// Config for the Configuration subcommand
pub struct ConfigurationConfig {
    /// Which action should be executed on the config
    action: ConfigurationAction,

    /// Optional path for the create command
    endpoint: Option<String>,
}

/// Configuration module
pub struct ConfigurationModule;

/// Implementation of a module for configuration
#[async_trait(?Send)]
impl ModuleWithBaseAgent<ConfigurationConfig> for ConfigurationModule {
    async fn run(agent: &BaseAgent, config: ConfigurationConfig) {
        match config.action {
            ConfigurationAction::Create => {
                ConfigurationModule::create(agent, config.endpoint.unwrap())
            }
            ConfigurationAction::Edit => ConfigurationModule::edit(agent),
            ConfigurationAction::Output => ConfigurationModule::output(agent),
        };
        std::process::exit(0)
    }

    async fn register<'a>(agent: &BaseAgent, matches: &ArgMatches<'a>) {
        if let Some(matches_config) = matches.subcommand_matches("config") {
            if matches_config.is_present("initialise") {
                let endpoint = matches_config.value_of("endpoint").unwrap();
                let config = ConfigurationConfig {
                    action: ConfigurationAction::Create,
                    endpoint: Some(endpoint.to_string()),
                };
                ConfigurationModule::run(agent, config).await;
            }
            if matches_config.is_present("edit") {
                let config = ConfigurationConfig {
                    action: ConfigurationAction::Edit,
                    endpoint: None,
                };
                ConfigurationModule::run(agent, config).await;
            }
            let config = ConfigurationConfig {
                action: ConfigurationAction::Output,
                endpoint: None,
            };
            ConfigurationModule::run(agent, config).await;
        }
    }
}

impl ConfigurationModule {
    /// Create a new configuration file at the default location
    fn create(agent: &BaseAgent, endpoint: String) {
        if agent.configuration_path.exists() {
            agent
                .logger
                .log("Default configuration file already exists");
            return;
        }

        // Get the directories
        let prefix = agent.configuration_path.parent().unwrap();

        // create all the required directories
        std::fs::create_dir_all(prefix)
            .unwrap_or_else(|_| throw(Error::UnableToCreateConfigurationFile));

        // Create the configuration file
        std::fs::File::create(&agent.configuration_path)
            .unwrap_or_else(|_| throw(Error::UnableToCreateConfigurationFile));

        let content = format!("[Default]\n{}\n", endpoint);

        // Write the default configuration to the file
        std::fs::write(&agent.configuration_path, content)
            .unwrap_or_else(|_| throw(Error::UnableToCreateConfigurationFile));

        agent.logger.log(format!(
            "Configuration file has been initialised at: {:#?}",
            agent.configuration_path
        ));
    }

    /// Ouput the path to the config file and the content of the default configuration file
    fn output(agent: &BaseAgent) {
        let out = std::fs::read_to_string(&agent.configuration_path)
            .unwrap_or_else(|_| throw(Error::UnableToReadDefaultConfigurationFile));
        agent.logger.log(agent.configuration_path.to_str().unwrap());
        agent.logger.log(out);
    }

    /// Edit the default configuration file
    fn edit(_agent: &BaseAgent) {
        panic!("edit is unimplemented")
    }
}
