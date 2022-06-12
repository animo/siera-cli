use crate::error::Result;
use crate::error::{self, Error};
use crate::help_strings::HelpStrings;
use crate::utils::config::{get_config_path, Configuration, Environment};
use clap::{Args, Subcommand};
use colored::*;
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
pub async fn parse_configuration_args(options: &ConfigurationOptions) -> Result<()> {
    let config_path = get_config_path()?;
    match &options.commands {
        ConfigurationSubcommands::View => {
            log_debug!(
                "Loaded configuration from {}",
                config_path.display().to_string().bold()
            );
            let output = fs::read_to_string(&config_path).map_err(|err| {
                log_debug!("Failed to read config file: {}", err);
                Box::<dyn std::error::Error>::from(error::Error::CannotReadConfigurationFile)
            })?;
            log!("Configuration path: {}", config_path.display());
            log!("{}", output);
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
                let (environment, configuration) = Configuration::init(token.to_owned());
                Configuration::add(environment, configuration)?;
                log_info!(
                    "Successfully added the default agent at {}.",
                    config_path.display()
                );
                return Ok(());
            }
            log_debug!("Adding a new entry to the configuration file");
            let path = get_config_path()?;
            let endpoint = agent_url.to_owned().ok_or(Error::NoAgentURLSupplied)?;
            let environment = environment.to_owned().ok_or(Error::NoEnvironmentSupplied)?;
            let env = Environment {
                endpoint,
                api_key: api_key.to_owned(),
                auth_token: token.to_owned(),
                // TODO: this can only be aca-py or afj
                agent: agent.to_owned(),
            };
            log_info!(
                "Writing {}: {} to {}",
                environment.bold(),
                env,
                path.display()
            );
            Configuration::add(environment.clone(), env)?;
            log_info!(
                "Successfully Added agent {} at {}.",
                environment,
                config_path.display()
            );

            log_debug!("Written a new entry to the configuration",);
            Ok(())
        }
        ConfigurationSubcommands::Remove { environment } => {
            log_debug!(
                "{} environment {} from the configuration",
                "Removing".bold().red(),
                environment.bold()
            );
            Configuration::remove(environment.to_owned())?;
            log!(
                "{} {} from the configuration",
                "Removed".bold().red(),
                environment.bold()
            );
            Ok(())
        }
    }
}
