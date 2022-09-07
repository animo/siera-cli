use crate::error::{Error, Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use webhooks::listen;
use clap::{Args, Subcommand};
use std::str;
use crate::utils::config::{get_config_from_path, get_config_path};

/// Webhooks options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Schema)]
pub struct WebhooksOptions {
    /// All the subcommands of the Webhooks cli
    #[clap(subcommand)]
    pub commands: WebhooksSubcommands,
}

/// Webhooks subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Webhooks)]
pub enum WebhooksSubcommands {

    /// Listen for webhooks on provided url
    #[clap(about = HelpStrings::Webhooks)]
    Listen {
        /// Specifying the environment to use
        #[clap(long, short= 'e', help = HelpStrings::WebhooksEnvironment)]
        environment: Option<String>
    },
}

/// Subcommand Webhooks parser
pub async fn parse_webhooks_args(
    options: &WebhooksOptions,
) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());

    match &options.commands {
        WebhooksSubcommands::Listen {
            environment
        } => {
            let env = 
            environment.as_deref().unwrap_or("default");
            let config_path = match get_config_path() {
                Ok(c) => {
                    if c.exists() {
                        Some(c)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }.unwrap();
            let config = match get_config_from_path(&config_path) {
                Ok(c) => Some(c),
                Err(_) => None,
            }.unwrap();

            let env_config = config.configurations.get_key_value(env).ok_or_else(|| Error::InvalidEnvironment(env.into())).unwrap();
            let agent_url = env_config.1.endpoint.clone();

            loader.stop();
            listen(agent_url);
        }
    }
}
