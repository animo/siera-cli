use crate::error::Result;
use crate::error::{self, Error};
use crate::help_strings::HelpStrings;
use crate::utils::config::{get_config_path, Configuration, Environment};
use clap::{Args, Subcommand};
use colored::Colorize;
use std::fs;

/// Configuration options and flags
#[derive(Args)]
pub struct ConfigurationOptions {
    /// All the subcommands of the configuration cli
    #[clap(subcommand)]
    pub commands: ConfigurationSubcommands,
}

/// Configuration subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Configuration)]
pub enum ConfigurationSubcommands {
    /// View the configuration
    #[clap(about = HelpStrings::ConfigurationView)]
    View,

    /// Add a new agent to the configuration
    #[clap(about = HelpStrings::ConfigurationAdd)]
    Add {
        /// Use the default configuration
        #[clap(short, long, help = HelpStrings::ConfigurationAddDefault)]
        default: bool,

        /// Set the environment name
        #[clap(long, short, help = HelpStrings::Environment, conflicts_with = "default")]
        environment: Option<String>,

        /// The url of where the agent is located
        #[clap(long, short='u', help = HelpStrings::AgentURL, conflicts_with = "default")]
        agent_url: Option<String>,

        /// Api key used for authentication at the agent
        #[clap(long, short='k', help = HelpStrings::ApiKey, conflicts_with = "default")]
        api_key: Option<String>,

        /// Multi tenancy token for access to the wallet
        #[clap(long, short='t', help = HelpStrings::ConfigurationInitializeToken)]
        token: Option<String>,

        /// The url of where the agent is located
        #[clap(long, short='a', help = HelpStrings::Agent, conflicts_with = "default")]
        agent: Option<String>,
    },

    /// Remove an entry in your configuration
    #[clap(about = HelpStrings::ConfigurationRemove)]
    Remove {
        /// Environment that should be removed from the configuration
        #[clap(long, short, help = HelpStrings::ConfigurationRemoveEnvironment)]
        environment: String,
    },
}

/// Subcommand configuration parser
pub fn parse_configuration_args(options: &ConfigurationOptions) -> Result<()> {
    let config_path = get_config_path()?;
    match &options.commands {
        ConfigurationSubcommands::View => {
            debug!({
                "message":
                    format!(
                        "Loaded configuration from {}",
                        config_path.to_string_lossy()
                    )
            });
            let output = fs::read_to_string(&config_path).map_err(|error| {
                debug!({ "message": "Failed to read config file", "error": error.to_string() });
                Box::<dyn std::error::Error>::from(error::Error::CannotReadConfigurationFile)
            })?;
            info!({ "coniguration_path": config_path });
            log!({ "output": output });
            Ok(())
        }
        ConfigurationSubcommands::Add {
            default,
            environment,
            agent_url,
            api_key,
            token,
            agent,
        } => {
            if *default {
                let (environment, configuration) = Configuration::init(token.clone());
                Configuration::add(environment, configuration)?;
                log!({
                    "message": "Successfully added the default agent",
                    "configuration_path": config_path
                });
                return Ok(());
            }
            debug!({ "message": "Adding a new entry to the configuration file"});
            let path = get_config_path()?;
            let endpoint = agent_url.clone().ok_or(Error::NoAgentURLSupplied)?;
            let environment = environment.clone().ok_or(Error::NoEnvironmentSupplied)?;
            let env = Environment {
                endpoint,
                api_key: api_key.clone(),
                auth_token: token.clone(),
                // TODO: this can only be aca-py or afj
                agent: agent.clone(),
            };
            log!({
                "message":
                    format!(
                        "Writing {}: {} to {}",
                        environment.bold(),
                        env,
                        path.display()
                    )
            });
            Configuration::add(environment.clone(), env)?;
            log!({
                "message":
                    format!(
                        "Successfully Added agent {} at {}.",
                        environment,
                        config_path.display()
                    )
            });

            debug!({ "message": "Written a new entry to the configuration"});
            Ok(())
        }
        ConfigurationSubcommands::Remove { environment } => {
            log!({
                "message":
                    format!(
                        "{} environment {} from the configuration",
                        "Removing".bold().red(),
                        environment.bold()
                    )
            });
            Configuration::remove(environment.clone())?;
            log!({
                "message":
                    format!(
                        "{} {} from the configuration",
                        "Removed".bold().red(),
                        environment.bold()
                    )
            });
            Ok(())
        }
    }
}
