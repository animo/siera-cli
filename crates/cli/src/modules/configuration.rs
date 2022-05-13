use crate::error::Result;
use crate::error::{self, Error};
use crate::help_strings::HelpStrings;
use crate::utils::config::{get_config_path, Configuration, Environment};
use clap::{Args, Subcommand};
use colored::*;
use log::{debug, info};
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
        #[clap(long, short, help = HelpStrings::ApiKey, conflicts_with = "default")]
        api_key: Option<String>,

        /// Multi tenancy token for access to the wallet
        #[clap(long, short='t', help = HelpStrings::ConfigurationInitializeToken)]
        token: Option<String>,
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
pub async fn parse_configuration_args(options: &ConfigurationOptions) -> Result<()> {
    let config_path = get_config_path()?;
    match &options.commands {
        ConfigurationSubcommands::View => {
            debug!(
                "Loaded configuration from {}",
                String::from(config_path.to_str().unwrap()).bold()
            );
            let output = fs::read_to_string(&config_path).map_err(|err| {
                debug!("Failed to read config file: {}", err);
                Box::<dyn std::error::Error>::from(error::Error::CannotReadConfigurationFile)
            })?;
            println!("Configuration path: {:?}", config_path);
            println!("{}", output);
            Ok(())
        }
        ConfigurationSubcommands::Add {
            default,
            environment,
            agent_url,
            api_key,
            token,
        } => {
            if *default {
                let (environment, configuration) = Configuration::init(token.to_owned());
                Configuration::add(environment, configuration)?;
                println!(
                    "{} the default agent at {}.",
                    "Added".cyan(),
                    config_path.display()
                );
                return Ok(());
            }
            debug!("{} a new entry to the configuration file", "Adding".cyan());
            let path = get_config_path()?;
            let endpoint = agent_url.to_owned().ok_or(Error::NoAgentURLSupplied)?;
            let environment = environment.to_owned().ok_or(Error::NoEnvironmentSupplied)?;
            let env = Environment {
                endpoint,
                api_key: api_key.to_owned(),
                auth_token: token.to_owned(),
            };
            info!(
                "{} {}, {:#?} to {:#?}",
                "Writing".cyan(),
                environment,
                env,
                path
            );
            Configuration::add(environment.clone(), env)?;
            println!(
                "{} agent {} at {}.",
                "Added".cyan(),
                environment,
                config_path.display()
            );

            debug!("{} a new entry to the configuration", "Written".green());
            Ok(())
        }
        ConfigurationSubcommands::Remove { environment } => {
            debug!(
                "{} environment {} from the configuration",
                "Removing".bold().red(),
                environment.bold()
            );
            Configuration::remove(environment.to_owned())?;
            println!(
                "{} {} from the configuration",
                "Removed".bold().red(),
                environment.bold()
            );
            Ok(())
        }
    }
}
