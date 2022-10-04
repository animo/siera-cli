use crate::error::Result;
use crate::help_strings::HelpStrings;
use clap::{Args, Subcommand};
use colored::Colorize;
use siera_agent::modules::webhook::WebhookModule;
use siera_logger::pretty_stringify_obj;

/// Webhook options and flags
#[derive(Args)]
#[clap(about = HelpStrings::Schema)]
pub struct WebhookOptions {
    /// All the subcommands of the Webhook cli
    #[clap(subcommand)]
    pub commands: WebhookSubcommands,
}

/// Webhook subcommands
#[derive(Subcommand, Debug)]
#[clap(about = HelpStrings::Webhook)]
pub enum WebhookSubcommands {
    /// Listen for webhook on provided url
    #[clap(about = HelpStrings::Webhook)]
    Listen {},
}

/// Subcommand webhook parser
pub async fn parse_webhook_args(agent: impl WebhookModule + Send + Sync) -> Result<()> {
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
