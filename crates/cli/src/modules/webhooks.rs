use crate::error::Result;
use crate::help_strings::HelpStrings;
use agent::modules::webhooks::WebhooksModule;
use clap::{Args, Subcommand};
use colored::Colorize;
use logger::pretty_stringify_obj;

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
    Listen {},
}

/// Subcommand webhooks parser
pub async fn parse_webhooks_args(agent: impl WebhooksModule + Send + Sync) -> Result<()> {
    agent
        .listen(|event| {
            let topic = event.get("topic");
            let incoming_webhook_message = topic.map_or_else(
                || format!("{}:", "Received hook".green()),
                |t| {
                    format!(
                        "{}: (topic: {})",
                        "Received hook".green(),
                        t.to_string().blue()
                    )
                },
            );
            log!("{}", incoming_webhook_message);
            log!("{}", pretty_stringify_obj(event));
        })
        .await
}
