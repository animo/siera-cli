use crate::error::{Result};
use crate::help_strings::HelpStrings;
use crate::utils::loader::{Loader, LoaderVariant};
use webhooks::listen;
use clap::{Args, Subcommand};
use std::str;

/// Oob options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Schema)]
pub struct WebhooksOptions {
    /// All the subcommands of the Oob cli
    #[clap(subcommand)]
    pub commands: WebhooksSubcommands,
}

/// Oob subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Webhooks)]
pub enum WebhooksSubcommands {

    /// Listen for webhooks on provided url
    #[clap(about = HelpStrings::Webhooks)]
    Listen {
        /// Invitation url
        #[clap(long, short, help = HelpStrings::WebhooksUrl)]
        url: Option<String>,
    },
}

/// Subcommand Oob parser
pub async fn parse_webhooks_args(
    options: &WebhooksOptions,
) -> Result<()> {
    let loader = Loader::start(&LoaderVariant::default());

    match &options.commands {
        WebhooksSubcommands::Listen {
            url,
        } => Ok({
            loader.stop();
            listen(url);
        })
    }
}
